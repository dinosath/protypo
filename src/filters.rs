use cruet::string::{pluralize, singularize};
use heck::{ToKebabCase, ToLowerCamelCase, ToPascalCase, ToSnakeCase};

pub fn snake_case(value: String) -> String {
    value.to_snake_case()
}

pub fn camel_case(value: String) -> String {
    value.to_lower_camel_case()
}

pub fn kebab_case(value: String) -> String {
    value.to_kebab_case()
}

pub fn pascal_case(value: String) -> String {
    value.to_pascal_case()
}

pub fn lower_camel_case(value: String) -> String {
    value.to_lower_camel_case()
}

pub fn plural(value: String) -> String {
    pluralize::to_plural(&value)
}

pub fn singular(value: String) -> String {
    singularize::to_singular(&value)
}