use std::iter::Peekable;

use crate::data_types::Type;
use crate::treewalk::ast::*;

#[derive(Copy, Clone)]
pub enum TransformTo {
    Flow,
    Typescript,
}

enum JSONTags {
    FieldWithTags(String),
    OmitField,
}

impl TransformTo {
    fn new_interface(self, name: &str) -> Vec<&str> {
        match self {
            TransformTo::Flow => vec!["export type ", name, " ="],
            TransformTo::Typescript => vec!["export interface ", name],
        }
    }
}
pub fn interpret(tokens: &[GoStruct], transform_to: TransformTo) -> String {
    let mut peekable_tokens = tokens.iter().peekable();
    let mut target = String::from("");
    while let Some(derived_str) = interpret_struct(&mut peekable_tokens, &transform_to) {
        target.push_str(&derived_str);
    }
    target
}

fn interpret_struct<'a, I>(tokens: &mut Peekable<I>, transform_to: &TransformTo) -> Option<String>
where
    I: Iterator<Item = &'a GoStruct>,
{
    match tokens.peek() {
        Some(&GoStruct::StructDefinition(ref s)) => {
            let _ = tokens.next();
            let mut interface = transform_to.new_interface(&s.name);

            let body = &interpret_struct_body(&s.body);
            let mut struct_body = vec![" { ", body, "};"];
            interface.append(&mut struct_body);
            Some(interface.into_iter().collect::<String>())
        }
        Some(_) => {
            let _ = tokens.next();
            Some(format!(""))
        }
        _ => None,
    }
}

fn interpret_struct_body(body: &GoStruct) -> String {
    let mut struct_body: Vec<String> = vec![];
    if let GoStruct::Block(ref body) = body {
        for statement in &body.statements {
            match statement {
                GoStruct::FieldNameWithTypeOnly(name, field_type) => {
                    struct_body.push(name.to_string());
                    let field_type = format!("?: {}; ", field_type);
                    struct_body.push(field_type);
                }
                GoStruct::FieldWithJSONTags(name, field_type, json) => {
                    let json_tags = interpret_json_tags(name.to_string(), *field_type, json);
                    match json_tags {
                        JSONTags::OmitField => {}
                        JSONTags::FieldWithTags(json_tags) => {
                            let json_tags = format!("{}; ", json_tags);
                            struct_body.push(json_tags);
                        }
                    }
                }
                GoStruct::FieldNameOnly(name) => {
                    let field_name_only = format!("... {};", name);
                    struct_body.push(field_name_only);
                }
                GoStruct::FieldWithListAndType(name, field_type) => {
                    let field_with_type_list = format!("{}:{}[];", name, field_type);
                    struct_body.push(field_with_type_list)
                }
                GoStruct::FieldWithListTypeAndJSONTags(name, field_type, json) => {
                    let json_list_props = interpret_json_tags(name.to_string(), *field_type, json);
                    match json_list_props {
                        JSONTags::OmitField => {}
                        JSONTags::FieldWithTags(json_list_props) => {
                            let json_list_props = format!("{}[];", json_list_props);
                            struct_body.push(json_list_props);
                        }
                    }
                }
                GoStruct::FieldWithIdentifierAndJSONTags(name, literaltype, json) => {
                    let identifier = interpret_json_tags_with_custom_field_type(
                        name.to_string(),
                        literaltype.to_string(),
                        json,
                    );
                    match identifier {
                        JSONTags::FieldWithTags(identifier) => {
                            let identifier = format!("{}; ", identifier);
                            struct_body.push(identifier);
                        }
                        JSONTags::OmitField => {}
                    }
                }
                GoStruct::FieldWithIdentifierTypeOnly(name, literaltype) => {
                    let field_with_literal_type = format!("{}: {}; ", name, literaltype);
                    struct_body.push(field_with_literal_type);
                }
                GoStruct::FieldWithCustomListIdentifier(name, customidentifier) => {
                    let field_with_custom_list_identifier =
                        format!("{}: {}; ", name, customidentifier);
                    struct_body.push(field_with_custom_list_identifier);
                }
                GoStruct::FieldWithCustomListIdentifierAndJSONTags(
                    name,
                    customidentifier,
                    json,
                ) => {
                    let field_with_custom_type = interpret_json_tags_with_custom_field_type(
                        name.to_string(),
                        customidentifier.to_string(),
                        json,
                    );
                    match field_with_custom_type {
                        JSONTags::FieldWithTags(field_with_custom_type) => {
                            let field_with_custom_type = format!("{}[]; ", field_with_custom_type);
                            struct_body.push(field_with_custom_type);
                        }
                        JSONTags::OmitField => {}
                    }
                }
                _ => {}
            }
        }
    }
    struct_body.into_iter().collect()
}

fn interpret_json_tags(name: String, field_type: Type, json: &[GoStruct]) -> JSONTags {
    let mut name = name;
    let mut is_prop_required = "?:".to_owned();
    for st in json {
        match st {
            GoStruct::JSONName(specified_name) => name = specified_name.to_string(),
            GoStruct::Binding => is_prop_required = ":".to_string(),
            _ => {}
        }
    }
    if name == *"-" {
        return JSONTags::OmitField;
    }
    JSONTags::FieldWithTags(format!("{}{}{}", name, is_prop_required, field_type))
}

fn interpret_json_tags_with_custom_field_type(
    name: String,
    field_type: String,
    json: &[GoStruct],
) -> JSONTags {
    let mut name = name;
    let mut is_prop_required = "?:".to_owned();
    for st in json {
        match st {
            GoStruct::JSONName(specified_name) => name = specified_name.to_string(),
            GoStruct::Binding => is_prop_required = ":".to_string(),
            _ => {}
        }
    }
    if name == *"-" {
        return JSONTags::OmitField;
    }
    JSONTags::FieldWithTags(format!("{}{}{}", name, is_prop_required, field_type))
}
