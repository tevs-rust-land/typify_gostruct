use crate::ast::{DataType, Field, FieldType, TagKey, AST};

use super::{FieldTypeResult, Interpreter};

pub struct FlowInterpreter();

impl Interpreter for FlowInterpreter {
    fn interpret(&self, ast: Vec<crate::ast::AST>) -> String {
        let mut result = String::new();
        for item in ast {
            let struct_results = match item {
                AST::Declaration(declaration) => self.interpret_struct(declaration),
                AST::Error(_) => todo!(),
                AST::Field(_) => todo!(),
            };
            result.push_str(&struct_results)
        }
        result
    }

    fn interpret_struct(&self, declaration: Box<crate::ast::StructDeclaration>) -> String {
        let mut result = format!("type {} = ", declaration.name);
        result.push('{');

        for item in declaration.body {
            let field_result = match item {
                AST::Field(field) => self.interpret_field(field),
                AST::Error(_) => todo!(),
                AST::Declaration(_) => todo!(),
            };
            result.push_str(&field_result)
        }
        result.push('}');
        result
    }

    fn interpret_field(&self, field: crate::ast::Field) -> String {
        let mut result = String::new();
        let field_result = match field {
            Field::Blank => String::new(),
            Field::Plain(field_name, field_type) => {
                let field_type = self.convert_convert_field_type(field_type);
                match field_type {
                    FieldTypeResult::Normal(field_type) => {
                        format!("{} : {},", field_name.0, field_type)
                    }
                    FieldTypeResult::Embedded => format!("...{}", field_name.0),
                }
            }
            Field::WithTags(field_name, field_type, field_tags) => {
                let field_type = self.convert_convert_field_type(field_type);
                match field_type {
                    FieldTypeResult::Normal(field_type) => {
                        format!("{} : {},", field_name.0, field_type)
                    }
                    FieldTypeResult::Embedded => format!("...{}", field_name.0),
                }
            }
        };

        result.push_str(&field_result);
        result
    }

    fn convert_convert_field_type(&self, field_type: FieldType) -> FieldTypeResult {
        let single_of_list = match field_type {
            FieldType::List(_) => "[]",
            FieldType::One(_) => "",
        };
        let field_type = match field_type {
            FieldType::One(data_type) => self.get_field_type(data_type),
            FieldType::List(data_type) => self.get_field_type(data_type),
        };

        match field_type {
            FieldTypeResult::Normal(specified_type) => {
                FieldTypeResult::Normal(format!("{}{}", single_of_list, specified_type))
            }
            FieldTypeResult::Embedded => FieldTypeResult::Embedded,
        }
    }

    fn interpret_field_with_tags(
        &self,
        field_name: crate::ast::FieldName,
        field_type: FieldType,
        tags: std::collections::HashMap<crate::ast::TagKey, crate::ast::TagValue>,
    ) -> String {
        let mut field_name = field_name.0;
        let field_type = self.convert_convert_field_type(field_type);

        for (key, value) in &tags {
            if *key == TagKey("json".to_string()) {
                field_name = value.0.clone()
            }
        }
        match field_type {
            FieldTypeResult::Normal(field_type) => format!("{}:{}", field_name, field_type),
            _ => unreachable!(),
        }
    }
}

impl FlowInterpreter {
    pub fn new() -> Self {
        Self {}
    }
    fn get_field_type(&self, data_type: DataType) -> FieldTypeResult {
        match data_type {
            DataType::Number => FieldTypeResult::Normal("number".to_string()),
            DataType::String => FieldTypeResult::Normal("string".to_string()),
            DataType::Boolean => FieldTypeResult::Normal("boolean".to_string()),
            DataType::Custom(custom) => FieldTypeResult::Normal(custom),
            DataType::Embedded => FieldTypeResult::Embedded,
        }
    }
}
