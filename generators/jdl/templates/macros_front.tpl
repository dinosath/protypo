{% import "_macros.tpl" as macros -%}


{%- macro get_field(property) -%}
{% filter trim %}
    {% if property.type and property.type == "string" -%}
        {% if property.format and property.format == "uuid" -%}
            TextField
        {% elif property.format and property.format == "date-time" -%}
            DateField
        {% elif property.format and property.format == "date" -%}
            DateField
        {% elif property.format and property.format == "time" -%}
            TextField
        {% else -%}
            TextField
        {% endif -%}    
    {% elif property.type and property.type == "boolean" -%}
            BooleanField
    {% elif property.type and property.type == "integer" -%}
            NumberField
    {% elif property.type and property.type == "number" -%}
            NumberField
    {% elif property.enum %}
            TextField
    {% elif macros::relation_is_one_to_many(property=property)=='true' or macros::relation_is_many_to_many(property=property)=='true'  -%}
        {% set relation = macros::get_relation(property=property) | plural | snake_case -%}
            TextField
    {% elif macros::relation_is_many_to_one(property=property)=='true' -%}
        {% set relation = macros::get_relation(property=property) -%}
            ReferenceField reference="{{ relation | plural | kebab_case }}" label="{{ relation | pascal_case }}"
    {% elif macros::relation_is_many_to_many(property=property)=='true' -%}
        {% set relation = macros::get_relation(property=property) -%}
        ReferenceArrayField reference="{{ relation | plural | kebab_case }}" label="{{ relation | pascal_case }}"
    {% else -%}
            TextField
    {% endif -%}
{% endfilter %}
{%- endmacro -%}

{%- macro is_read_only(property) -%}
{% if property and property.readOnly -%}
readOnly
{% endif -%}
{%- endmacro -%}

{%- macro get_input_field(property) -%}
{% filter trim %}
    {% if property.type and property.type == "string" -%}
        {% if property.format and property.format == "uuid" -%}
            TextInput
        {% elif property.format and property.format == "date-time" -%}
            DateTimeInput
        {% elif property.format and property.format == "date" -%}
            DateInput
        {% elif property.format and property.format == "time" -%}
            TextInput
        {% else -%}
            TextInput
        {% endif -%}    
    {% elif property.type and property.type == "boolean" -%}
        BooleanInput
    {% elif property.type and property.type == "integer" -%}
        NumberInput
    {% elif property.type and property.type == "number" -%}
        NumberInput
    {% elif property.enum %}
        SelectField choices={[
            {%- for enum in property.enum -%}
               { name: '{{ enum }}' }{%- if not loop.last -%},{% endif -%}
            {%- endfor -%}
            ]}
    {% elif macros::relation_is_many_to_one(property=property)=='true' -%}
        {% set relation = macros::get_relation(property=property) -%}
        ReferenceInput reference="{{ relation | plural | kebab_case }}" label="{{ relation | pascal_case }}"
    {% elif macros::relation_is_many_to_many(property=property)=='true' -%}
        {% set relation = macros::get_relation(property=property) -%}
        ReferenceArrayInput reference="{{ relation | plural | kebab_case }}" label="{{ relation | pascal_case }}"
    {% else -%}
        TextInput
    {% endif -%}
{% endfilter %}
{%- endmacro -%}


{%- macro get_all_properties_by_name(entity) -%}
{%- set_global properties = [] -%}
{% for name,property in entity.properties -%}
    {% if macros::relation_is_one_to_many(property=property)=='true' or macros::relation_is_many_to_many(property=property)=='true'  -%}
    {% continue -%}
    {% endif -%}
    {% if macros::relation_is_many_to_one(property=property)=='true'  -%}
    {% set relation = macros::get_relation(property=property) | camel_case | trim -%}
    {% set name = relation ~ " { id }" -%}
    {% endif -%}
    {%- set_global properties = properties | concat(with=name) -%}
{% endfor -%}
{{ properties | join(sep=" ") }}
{%- endmacro -%}

{%- macro source(name,property) -%}
{% filter trim %}
{{name|camel_case}}
{%- if macros::relation_is_many_to_one(property=property)=='true' -%}
.id
{%- endif -%}
{%- endfilter -%}
{%- endmacro -%}
