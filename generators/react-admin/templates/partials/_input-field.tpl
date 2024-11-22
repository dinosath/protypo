{%- import "_macros_front.tpl" as macros -%}
{%- import "_macros.tpl" as core -%}
{% set is_relation = core.relation_is_many_to_many(property=property)=='true' or core.relation_is_many_to_one(property=property)=='true' %}
            <{{ get_input_field(property=property) }} {{ is_read_only(property=property) }} source="{{ source(name=name,property=property)}}"
{%- if is_relation %}{{' '}}
{%- set relation = macros.get_relation(property=property) -%}
reference="{{ relation | plural | kebab_case }}" label="{{ relation | pascal_case }}">
                <AutocompleteInput label="{{ name }}" {{ validation(entity=entity,name=name,property=property) }}/>
            </{{ macros.get_input_field(property=property) }}>
{% else -%}
{{' '}}{{ macros.validation(entity=entity,name=name,property=property) }}/>
{% endif -%}