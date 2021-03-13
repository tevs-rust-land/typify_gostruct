use std::iter::Peekable;

use crate::data_types::Type;
use crate::treewalk::ast::*;

#[derive(Copy, Clone)]
pub enum TransformTo {
    Flow,
    Typescript,
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
            let result: String = interface.into_iter().collect();
            Some(result)
        }
        Some(_) => {
            let _ = tokens.next();

            Some("".to_string())
        }
        None => None,
    }
}

fn interpret_struct_body(body: &GoStruct) -> String {
    let mut struct_body = vec!["".to_owned()];
    if let GoStruct::Block(ref body) = body {
        for statement in &body.statements {
            match statement {
                GoStruct::StructNameWithTypeOnly(name, typ) => {
                    struct_body.push(name.to_string());
                    let data_type = match typ {
                        Type::TypeAny => "?:any; ",
                        Type::TypeNumber => "?:number; ",
                        Type::TypeString => "?:string; ",
                        Type::TypeNullNumber => ":number | null; ",
                        Type::TypeNullString => ":string | null; ",
                        Type::TypeBoolean => ":boolean; ",
                        Type::TypeDate => ":number; ",
                    };
                    struct_body.push(data_type.to_owned());
                }
                GoStruct::StructWithJSONTags(name, typ, json) => {
                    let json_tags = interpret_json_properties(name.to_string(), *typ, json);
                    struct_body.push(json_tags);
                }
                GoStruct::StructNameOnly(name) => {
                    let mut struct_name_only =
                        vec!["...".to_string(), name.to_string(), "; ".to_string()];
                    struct_body.append(&mut struct_name_only);
                }
                GoStruct::StructWithListAndType(name, typ) => {
                    let mut struct_with_type = vec![name.to_string()];
                    let list_type = match typ {
                        Type::TypeNumber => ":number[]; ",
                        Type::TypeString => ":string[]; ",
                        Type::TypeBoolean => ":boolean[]; ",
                        Type::TypeDate => ":number[]; ",
                        _ => "",
                    };
                    struct_with_type.push(list_type.to_string());
                    struct_body.append(&mut struct_with_type)
                }
                GoStruct::StructWithListTypeAndJSONTags(name, typ, json) => {
                    let json_list_props =
                        &interpret_json_list_properties(name.to_string(), *typ, json);
                    struct_body.push(json_list_props.to_string());
                }
                GoStruct::StructWithIdentifierAndJSONTags(name, literaltype, json) => {
                    let identifier = &interpret_json_with_identifier(
                        name.to_string(),
                        literaltype.to_string(),
                        json,
                    );
                    struct_body.push(identifier.to_string());
                }
                GoStruct::StructWithIdentifierTypeOnly(name, literaltype) => {
                    let name = name.to_owned();
                    let literaltype = literaltype.to_owned();
                    let mut struct_with_type_only =
                        vec![name, ":".to_string(), literaltype, "; ".to_string()];

                    struct_body.append(&mut struct_with_type_only);
                }
                GoStruct::StructWithCustomListIdentifier(name, customidentifier) => {
                    let name = name.to_owned();
                    let customidentifier = customidentifier.to_owned();
                    let mut struct_with_custom_list_identifier =
                        vec![name, ":".to_string(), customidentifier, "; ".to_string()];

                    struct_body.append(&mut struct_with_custom_list_identifier);
                }
                GoStruct::StructWithCustomListIdentifierAndJSONTags(
                    name,
                    customidentifier,
                    json,
                ) => {
                    let custom_identifier = interpret_json_with_custom_identifier(
                        name.to_string(),
                        customidentifier.to_string(),
                        json,
                    );
                    struct_body.push(custom_identifier);
                }
                _ => {}
            }
        }
    }
    struct_body.into_iter().collect()
}

fn interpret_json_properties(name: String, typ: Type, json: &[GoStruct]) -> String {
    let mut json_props = vec!["".to_owned()];
    let mut temp_name = name;
    let mut temp_binding_type = "?:".to_owned();
    let type_string = match typ {
        Type::TypeAny => "any; ",
        Type::TypeNumber => "number; ",
        Type::TypeString => "string; ",
        Type::TypeNullNumber => "number | null; ",
        Type::TypeNullString => "string | null; ",
        Type::TypeBoolean => "boolean; ",
        Type::TypeDate => "number; ",
    };
    for st in json {
        match st {
            GoStruct::JSONName(specified_name) => temp_name = specified_name.to_string(),
            GoStruct::Binding => temp_binding_type = ":".to_string(),
            _ => {}
        }
    }
    let mut attributes = vec![temp_name, temp_binding_type, type_string.to_string()];
    json_props.append(&mut attributes);
    json_props.into_iter().collect()
}

fn interpret_json_list_properties(name: String, typ: Type, json: &[GoStruct]) -> String {
    let mut json_props = vec!["".to_owned()];
    let mut temp_name = name;
    let mut temp_binding_type = "?:".to_owned();
    let type_string = match typ {
        Type::TypeNumber => "number[]; ",
        Type::TypeString => "string[]; ",
        Type::TypeBoolean => "boolean[]; ",
        Type::TypeDate => "Date[]; ",
        _ => "",
    };
    for st in json {
        match st {
            GoStruct::JSONName(specified_name) => temp_name = specified_name.to_string(),
            GoStruct::Binding => temp_binding_type = ":".to_string(),
            _ => {}
        }
    }

    let mut attributes = vec![temp_name, temp_binding_type, type_string.to_string()];
    json_props.append(&mut attributes);
    json_props.into_iter().collect()
}

fn interpret_json_with_identifier(name: String, typ: String, json: &[GoStruct]) -> String {
    let mut json_props = vec!["".to_owned()];
    let mut temp_name = name;
    let mut temp_binding_type = "?:".to_owned();
    for st in json {
        match st {
            GoStruct::JSONName(specified_name) => temp_name = specified_name.to_string(),
            GoStruct::Binding => temp_binding_type = ":".to_string(),
            _ => {}
        }
    }

    let mut attributes = vec![temp_name, temp_binding_type, typ];
    attributes.push("; ".to_string());

    json_props.append(&mut attributes);
    json_props.into_iter().collect()
}

fn interpret_json_with_custom_identifier(name: String, typ: String, json: &[GoStruct]) -> String {
    let mut json_props = vec!["".to_owned()];
    let mut temp_name = name;
    let mut temp_binding_type = "?:".to_owned();
    for st in json {
        match st {
            GoStruct::JSONName(specified_name) => temp_name = specified_name.to_string(),
            GoStruct::Binding => temp_binding_type = ":".to_string(),
            _ => {}
        }
    }

    let mut attributes = vec![temp_name, temp_binding_type, typ];
    attributes.push("[]; ".to_string());
    json_props.append(&mut attributes);
    json_props.into_iter().collect()
}
