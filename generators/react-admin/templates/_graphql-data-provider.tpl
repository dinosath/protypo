{% import "_macros_front.tpl" as macros -%}
import { ApolloClient, InMemoryCache, gql } from "@apollo/client";
import { omit } from "lodash";

const apiUrl = 'http://localhost:5150/api/graphql';

const client = new ApolloClient({
    uri: apiUrl,
    headers: { "x-graphql-token": "YYY" },
    cache: new InMemoryCache(),
    defaultOptions: {
        watchQuery: {
            fetchPolicy: 'no-cache',
            errorPolicy: 'ignore',
        },
        query: {
            fetchPolicy: 'no-cache',
            errorPolicy: 'all',
        },
    } 
});

//todo add nodes and ids in relationships
const fields = {
    {% for entity_name,entity in entities -%}
    {%- if not entity.properties -%}{%- continue -%}{%- endif -%}
    '{{ entity.title | plural | kebab_case }}': "id createdAt updatedAt {{macros::get_all_properties_by_name(entity=entity)}}"{%- if not loop.last -%},{% endif %}
    {% endfor %}
};

const pascal = {
    {% for entity_name,entity in entities -%}
    {%- if not entity.properties -%}{%- continue -%}{%- endif -%}
    '{{ entity.title | plural | kebab_case }}': "{{ entity.title | pascal_case }}"{%- if not loop.last -%},{% endif %}
    {% endfor %}
};

const camel = {
    {% for entity_name,entity in entities -%}
    {%- if not entity.properties -%}{%- continue -%}{%- endif -%}
    '{{ entity.title | plural | kebab_case }}': "{{ entity.title | camel_case }}"{%- if not loop.last -%},{% endif %}
    {% endfor %}
};

export const dataProvider = {
    getList: (resource,{ sort, pagination, filter, signal }, options) => {
        console.log(resource)
        console.log(options)
        const { field, order } = sort;
        const { page, perPage } = pagination;
        return client
            .query({
                query: gql`
            query ($limit: Int!, $offset: Int!, $orderBy: ${pascal[resource]}OrderInput) {
                ${camel[resource]}(pagination: { page: { limit: $limit, page: $offset } }, orderBy:$orderBy) {
                    nodes{
                        ${fields[resource]}
                    }
                    paginationInfo {
                      pages current offset total __typename
                    }
                    pageInfo {
                      hasPreviousPage hasNextPage startCursor endCursor __typename
                    }
                }
            }`,
                variables: {
                    limit: perPage,
                    offset: (page - 1) * perPage,
                    orderBy: { "id": "ASC" },
                    where: Object.keys(filter).reduce(
                        (prev, key) => ({
                            ...prev,
                            [key]: { _eq: filter[key] },
                        }),
                        {}
                    ),
                },
                context: {
                    fetchOptions: {
                        signal,
                    },
                },
            })
            .then((result) => ({
                data: result.data[camel[resource]].nodes,
                total: result.data[camel[resource]].paginationInfo.total,
            }));
    },
    getOne: (resource, params) => {
        return client
            .query({
                query: gql`
            query ($id: Int!) {
                ${resource}(id: $id) {
                    ${fields[resource]}
                }
            }`,
                variables: {
                    id: params.id,
                },
                context: {
                    fetchOptions: {
                        signal: params.signal,
                    },
                },
            })
            .then((result) => ({ data: result.data[`${resource}`] }));
    },
    getMany: (resource, params) => {
        return client
            .query({
                query: gql`
            query ($where: ${resource}_bool_exp) {
                ${resource}(where: $where) {
                    ${fields[resource]}
                }
            }`,
                variables: {
                    where: {
                        id: { _in: params.ids },
                    },
                },
                context: {
                    fetchOptions: {
                        signal: params.signal,
                    },
                },
            })
            .then((result) => ({ data: result.data[resource] }));
    },
    getManyReference: (
        resource,
        { target, id, sort, pagination, filter, signal }
    ) => {
        const { field, order } = sort;
        const { page, perPage } = pagination;
        return client
            .query({
                query: gql`
            query ($limit: Int, $offset: Int, $order_by: [${resource}_order_by!], $where: ${resource}_bool_exp) {
                ${resource}(limit: $limit, offset: $offset, order_by: $order_by, where: $where) {
                    ${fields[resource]}
                }
                ${resource}_aggregate(where: $where) {
                    aggregate {
                        count
                    }
                }
            }`,
                variables: {
                    limit: perPage,
                    offset: (page - 1) * perPage,
                    order_by: { [field]: order.toLowerCase() },
                    where: Object.keys(filter).reduce(
                        (prev, key) => ({
                            ...prev,
                            [key]: { _eq: filter[key] },
                        }),
                        { [target]: { _eq: id } }
                    ),
                },
                context: {
                    fetchOptions: {
                        signal,
                    },
                },
            })
            .then((result) => ({
                data: result.data[resource],
                total: result.data[`${resource}_aggregate`].aggregate.count,
            }));
    },
    create: (resource, params) => {
        return client
            .mutate({
                mutation: gql`
            mutation ($data: ${resource}_insert_input!) {
                insert_${resource}_one(object: $data) {
                    ${fields[resource]}
                }
            }`,
                variables: {
                    data: omit(params.data, ['__typename']),
                },
            })
            .then((result) => ({
                data: result.data[`insert_${resource}_one`],
            }));
    },
    update: (resource, params) => {
        return client
            .mutate({
                mutation: gql`
            mutation ($id: Int!, $data: ${resource}_set_input!) {
                update_${resource}_by_pk(pk_columns: { id: $id }, _set: $data) {
                    ${fields[resource]}
                }
            }`,
                variables: {
                    id: params.id,
                    data: omit(params.data, ['__typename']),
                },
            })
            .then((result) => ({
                data: result.data[`update_${resource}_by_pk`],
            }));
    },
    updateMany: (resource, params) => {
        return client
            .mutate({
                mutation: gql`
            mutation ($where: ${resource}_bool_exp!, $data: ${resource}_set_input!) {
                update_${resource}(where: $where, _set: $data) {
                    affected_rows
                }
            }`,
                variables: {
                    where: {
                        id: { _in: params.ids },
                    },
                    data: omit(params.data, ['__typename']),
                },
            })
            .then((result) => ({
                data: params.ids,
            }));
    },
    delete: (resource, params) => {
        return client
            .mutate({
                mutation: gql`
            mutation ($id: Int!) {
                delete_${resource}_by_pk(id: $id) {
                    ${fields[resource]}
                }
            }`,
                variables: {
                    id: params.id,
                },
            })
            .then((result) => ({
                data: result.data[`delete_${resource}_by_pk`],
            }));
    },
    deleteMany: (resource, params) => {
        return client
            .mutate({
                mutation: gql`
            mutation ($where: ${resource}_bool_exp!) {
                delete_${resource}(where: $where) {
                    affected_rows
                }
            }`,
                variables: {
                    where: {
                        id: { _in: params.ids },
                    },
                },
            })
            .then((result) => ({
                data: params.ids,
            }));
    },
};