import {DataProvider, fetchUtils} from "react-admin";
import {stringify} from "query-string";

function fetchJson(url: any, options: any) {
    options.user = {
        authenticated: true,
        token: localStorage.getItem('token')
    };
    console.log('options:' + JSON.stringify(options));
    return fetchUtils.fetchJson(url, options);
}

const apiUrl = '/api';


export const dataProvider: DataProvider = {
    getList: async (resource, params) => {
        console.log('[getList] resource: ' + JSON.stringify(resource));
        console.log('[getList] params: ' + JSON.stringify(params));

        const {page, perPage} = params.pagination;
        const {field, order} = params.sort;
        const query = {
            sort: `${field},${order}`,
            range: JSON.stringify([(page - 1) * perPage, page * perPage - 1]),
            filter: JSON.stringify(params.filter),
        };
        const url = `${apiUrl}/${resource}?${stringify(query)}`;


        const {json, headers} = await fetchJson(url, {signal: params.signal});
        const contentRange = headers.get('content-range');
        return {
            data: json,
            total: contentRange ? parseInt(contentRange.split('/').pop(), 10) : json.length,
        };
    },

    getOne: async (resource, params) => {
        console.log('[getOne] resource: ' + JSON.stringify(resource));
        console.log('[getOne] params: ' + JSON.stringify(params));
        return fetchJson(`${apiUrl}/${resource}/${params.id}`, {}).then(({json}) => ({
            data: json,
        }))
    },

    getMany: async (resource, params) => {
        console.log('[getMany] resource: ' + JSON.stringify(resource));
        console.log('[getMany] params: ' + JSON.stringify(params));
        const query = {
            filter: JSON.stringify({id: params.ids}),
        };
        const url = `${apiUrl}/${resource}?${stringify(query)}`;
        return fetchJson(url, {}).then(({json}) => ({data: json}));
    },

    getManyReference: async (resource, params) => {
        console.log('[getManyReference] resource: ' + JSON.stringify(resource));
        console.log('[getManyReference] params: ' + JSON.stringify(params));
        const {page, perPage} = params.pagination;
        const {field, order} = params.sort;
        const query = {
            sort: `${field},${order}`,
            range: JSON.stringify([(page - 1) * perPage, page * perPage - 1]),
            filter: JSON.stringify({
                ...params.filter,
                [params.target]: params.id,
            }),
        };
        const url = `${apiUrl}/${resource}?${stringify(query)}`;

        return fetchJson(url).then(({headers, json}) => ({
            data: json,
            total: parseInt((headers.get('content-range') || "0").split('/').pop() || '0', 10),
        }));
    },

    update: async (resource, params) => {
        console.log('[update] resource: ' + JSON.stringify(resource));
        console.log('[update] params: ' + JSON.stringify(params));
        return fetchJson(`${apiUrl}/${resource}/${params.id}`, {
            method: 'PUT',
            body: JSON.stringify(params.data),
        }).then(({json}) => ({data: json}))
    },

    updateMany: async (resource, params) => {
        console.log('[updateMany] resource: ' + JSON.stringify(resource));
        console.log('[updateMany] params: ' + JSON.stringify(params));
        const query = {
            filter: JSON.stringify({id: params.ids}),
        };
        return fetchJson(`${apiUrl}/${resource}?${stringify(query)}`, {
            method: 'PUT',
            body: JSON.stringify(params.data),
        }).then(({json}) => ({data: json}));
    },

    create: async (resource, params) => {
        console.log('[create] resource: ' + JSON.stringify(resource));
        console.log('[create] params: ' + JSON.stringify(params));
        return fetchJson(`${apiUrl}/${resource}`, {
            method: 'POST',
            body: JSON.stringify(params.data),
        }).then(({json}) => ({
            data: {...params.data, id: json.id} as any,
        }))
    },

    delete: async (resource, params) => {
        console.log('[delete] resource: ' + JSON.stringify(resource));
        console.log('[delete] params: ' + JSON.stringify(params));
        return fetchJson(`${apiUrl}/${resource}/${params.id}`, {
            method: 'DELETE',
        }).then(({json}) => ({data: json}))
    },

    deleteMany: async (resource, params) => {
        console.log('[delete] resource: ' + JSON.stringify(resource));
        console.log('[delete] params: ' + JSON.stringify(params));
        const query = {
            filter: JSON.stringify({id: params.ids}),
        };
        let deletePromises = params.ids.map(id => {
            fetchJson(`${apiUrl}/${resource}/${id}`, {
                method: 'DELETE',
            })
        });
        return Promise.all(deletePromises)
            .then((responses) => {
                // Once all deletions are done, return the ids of the deleted items
                return {data: params.ids};
            })
            .catch(error => {
                console.error('[deleteMany] Error:', error);
                throw new Error(error);
            });
    }
};