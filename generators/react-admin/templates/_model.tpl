{# generate model ui #}
{% import "_macros_front.tpl" as macros %}
{% import "_macros.tpl" as core %}

import { Show, SimpleShowLayout, Create, List, DatagridConfigurable, TextField, BooleanField, BooleanInput, DateField, ReferenceField, EditButton, Edit, SimpleForm, ReferenceInput, TextInput,DateInput, DateTimeInput, ReferenceArrayInput, NumberField, NumberInput, required, AutocompleteInput } from "react-admin";
import { ListActions } from "../components/common/action-buttons/listActions.tsx";
import { UserBulkActionButtons } from "../components/common/action-buttons/bulkListActions.tsx";
import {ShowActionButtons} from "../components/common/action-buttons/showActionButtons.tsx";
import {EditActionButtons} from "../components/common/action-buttons/editActionButtons.tsx";


const {{ entity.title | camel_case }}Filters = [
    <TextInput source="q" label="Search" alwaysOn />,
];


export const {{ entity.title | pascal_case }}List = () => (
    <List filters={ {{ entity.title | camel_case }}Filters} actions={<ListActions />} >
        <DatagridConfigurable>
            <TextField source="id" />
            {% if extraFields and 'createdAt' in extraFields -%}<TextField source="created_at" />{% endif -%}
            {% if extraFields and 'updatedAt' in extraFields -%}<TextField source="updated_at" />{% endif -%}
            {% for name,property in entity.properties -%}
            {% if core::relation_is_one_to_many(property=property)=='true' -%}{% continue -%}{% endif -%}
            <{{ macros::get_field(property=property) }} source="{{ macros::source(name=name,property=property)}}" />
            {% endfor %}
        </DatagridConfigurable>
    </List>
);

export const {{ entity.title | pascal_case }}Show = () => (
      <Show actions={<ShowActionButtons/>}>
        <SimpleShowLayout>
            <TextField source="id" />
            {% if extraFields and 'createdAt' in extraFields -%}<TextField source="created_at" />{% endif -%}
            {% if extraFields and 'updatedAt' in extraFields -%}<TextField source="updated_at" />{% endif -%}
            {% for name,property in entity.properties -%}
                {% if core::relation_is_one_to_many(property=property)=='true' -%}{% continue -%}{% endif -%}
                <{{ macros::get_field(property=property) }} source="{{ macros::source(name=name,property=property)}}" />
            {% endfor %}
        </SimpleShowLayout>
      </Show>
);

export const {{ entity.title | pascal_case }}Edit = () => (
    <Edit actions={<EditActionButtons/>}>
        <SimpleForm>
            <TextInput readOnly source="id" />
            {% if extraFields and 'createdAt' in extraFields -%}<DateTimeInput readOnly source="created_at" />{% endif -%}
            {% if extraFields and 'updatedAt' in extraFields -%}<DateTimeInput readOnly source="updated_at" />{% endif -%}
            {% for name,property in entity.properties -%}
                {% if core::relation_is_one_to_many(property=property)=='true' -%}{% continue -%}{% endif -%}
                {% include "_input-field.tpl" -%}
            {% endfor %}
        </SimpleForm>
    </Edit>
);

export const {{ entity.title | pascal_case }}Create = () => (
      <Create>
        <SimpleForm>
            {% for name,property in entity.properties -%}
                {% if core::relation_is_one_to_many(property=property)=='true' -%}{% continue -%}{% endif -%}
                {% include "_input-field.tpl" -%}
            {% endfor %}
        </SimpleForm>
      </Create>
);

export default {
    list: {{ entity.title | pascal_case }}List,
    show: {{ entity.title | pascal_case }}Show,
    edit: {{ entity.title | pascal_case }}Edit,
    create: {{ entity.title | pascal_case }}Create,
};