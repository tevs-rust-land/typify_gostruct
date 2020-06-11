use std::iter::Peekable;

use crate::scanner::scanner::*;
use crate::treewalk::ast::*;

pub fn interpret(tokens: &[GoStruct]) -> String {
    let mut peekable_tokens = tokens.iter().peekable();
    let mut target = String::from("");
    while let Some(derived_str) = interpret_struct(&mut peekable_tokens) {
        target.push_str(&derived_str);
    }

    target
}

fn interpret_struct<'a, I>(tokens: &mut Peekable<I>) -> Option<String>
where
    I: Iterator<Item = &'a GoStruct>,
{
    match tokens.peek() {
        Some(&GoStruct::StructDefinition(ref s)) => {
            let _ = tokens.next();
            let temp_string = &("type ".to_string() + &s.name + " = ");
            let temp_string = &(temp_string.to_string() + "{ ");
            let temp_string = &(temp_string.to_string() + &interpret_struct_body(&s.body));
            let temp_string = &(temp_string.to_string() + "};");

            Some(temp_string.to_string())
        }
        Some(_) => {
            let _ = tokens.next();

            Some("".to_string())
        }
        None => None,
    }
}

fn interpret_struct_body(body: &GoStruct) -> String {
    let mut res = "".to_owned();

    match body {
        GoStruct::Block(ref body) => {
            for statement in &body.statements {
                match statement {
                    GoStruct::StructNameWithTypeOnly(name, typ) => {
                        res.push_str(name);
                        match typ {
                            DataTypeEnum::TypeAny => res.push_str(":?any; "),
                            DataTypeEnum::TypeNumber => res.push_str(":?number; "),
                            DataTypeEnum::TypeString => res.push_str(":?string; "),
                            DataTypeEnum::TypeNullNumber => res.push_str(":number | null; "),
                            DataTypeEnum::TypeNullString => res.push_str(":string | number; "),
                            DataTypeEnum::TypeBoolean => res.push_str(":boolean; "),
                            DataTypeEnum::TypeDate => res.push_str(":Date; "),
                        }
                    }
                    GoStruct::StructWithJSONTags(name, typ, json) => {
                        res.push_str(&interpret_json_properties(name.to_string(), *typ, json));
                    }
                    GoStruct::StructNameOnly(name) => {
                        res.push_str(&("...".to_string()));
                        res.push_str(name);
                        res.push_str(&("; ".to_string()));
                    }
                    GoStruct::StructWithListAndType(name, typ) => {
                        res.push_str(name);
                        match typ {
                            DataTypeEnum::TypeNumber => res.push_str(":number[]; "),
                            DataTypeEnum::TypeString => res.push_str(":string[]; "),
                            DataTypeEnum::TypeBoolean => res.push_str(":boolean[]; "),
                            DataTypeEnum::TypeDate => res.push_str(":Date[]; "),
                            _ => {}
                        }
                    }
                    GoStruct::StructWithListTypeAndJSONTags(name, typ, json) => {
                        res.push_str(&interpret_json_list_properties(
                            name.to_string(),
                            *typ,
                            json,
                        ));
                    }
                    GoStruct::StructWithIdentifierAndJSONTags(name, literaltype, json) => res
                        .push_str(&interpret_json_with_identifier(
                            name.to_string(),
                            literaltype.to_string(),
                            json,
                        )),
                    GoStruct::StructWithIdentifierTypeOnly(name, literaltype) => {
                        res.push_str(name);
                        res.push_str(":");
                        res.push_str(literaltype);
                        res.push_str("; ");
                    }
                    GoStruct::StructWithCustomListIdentifier(name, customidentifier) => {
                        res.push_str(name);
                        res.push_str(":");
                        res.push_str(customidentifier);
                        res.push_str("[]; ");
                    }
                    GoStruct::StructWithCustomListIdentifierAndJSONTags(
                        name,
                        customidentifier,
                        json,
                    ) => res.push_str(&interpret_json_with_custom_identifier(
                        name.to_string(),
                        customidentifier.to_string(),
                        json,
                    )),
                    _ => {}
                }
            }
        }
        _ => {}
    }
    res.to_string()
}

fn interpret_json_properties(name: String, typ: DataTypeEnum, json: &Vec<GoStruct>) -> String {
    let mut json_prop = "".to_owned();
    let mut temp_name = name.to_owned();
    let mut temp_binding_type = ":?".to_owned();
    let type_string = match typ {
        DataTypeEnum::TypeAny => "any; ",
        DataTypeEnum::TypeNumber => "number; ",
        DataTypeEnum::TypeString => "string; ",
        DataTypeEnum::TypeNullNumber => "number | null; ",
        DataTypeEnum::TypeNullString => "string | number; ",
        DataTypeEnum::TypeBoolean => "boolean; ",
        DataTypeEnum::TypeDate => "Date; ",
    };
    for st in json {
        match st {
            GoStruct::JSONName(specified_name) => temp_name = specified_name.to_string(),
            GoStruct::Binding => temp_binding_type = ":".to_string(),
            _ => {}
        }
    }

    json_prop.push_str(&temp_name);
    json_prop.push_str(&temp_binding_type);
    json_prop.push_str(&type_string);

    json_prop
}

fn interpret_json_list_properties(name: String, typ: DataTypeEnum, json: &Vec<GoStruct>) -> String {
    let mut json_prop = "".to_owned();
    let mut temp_name = name.to_owned();
    let mut temp_binding_type = ":?".to_owned();
    let type_string = match typ {
        DataTypeEnum::TypeNumber => "number[]; ",
        DataTypeEnum::TypeString => "string[]; ",
        DataTypeEnum::TypeBoolean => "boolean[]; ",
        DataTypeEnum::TypeDate => "Date[]; ",
        _ => "",
    };
    for st in json {
        match st {
            GoStruct::JSONName(specified_name) => temp_name = specified_name.to_string(),
            GoStruct::Binding => temp_binding_type = ":".to_string(),
            _ => {}
        }
    }

    json_prop.push_str(&temp_name);
    json_prop.push_str(&temp_binding_type);
    json_prop.push_str(&type_string);

    json_prop
}

fn interpret_json_with_identifier(name: String, typ: String, json: &Vec<GoStruct>) -> String {
    let mut json_prop = "".to_owned();
    let mut temp_name = name.to_owned();
    let mut temp_binding_type = ":?".to_owned();
    for st in json {
        match st {
            GoStruct::JSONName(specified_name) => temp_name = specified_name.to_string(),
            GoStruct::Binding => temp_binding_type = ":".to_string(),
            _ => {}
        }
    }
    json_prop.push_str(&temp_name);
    json_prop.push_str(&temp_binding_type);
    json_prop.push_str(&typ);
    json_prop.push_str("; ");

    json_prop
}

fn interpret_json_with_custom_identifier(
    name: String,
    typ: String,
    json: &Vec<GoStruct>,
) -> String {
    let mut json_prop = "".to_owned();
    let mut temp_name = name.to_owned();
    let mut temp_binding_type = ":?".to_owned();
    for st in json {
        match st {
            GoStruct::JSONName(specified_name) => temp_name = specified_name.to_string(),
            GoStruct::Binding => temp_binding_type = ":".to_string(),
            _ => {}
        }
    }
    json_prop.push_str(&temp_name);
    json_prop.push_str(&temp_binding_type);
    json_prop.push_str(&typ);
    json_prop.push_str("[]; ");

    json_prop
}
