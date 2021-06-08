use std::collections::HashMap;

use crate::ast::{Field, FieldName, FieldType, StructDeclaration, TagKey, TagValue, AST};

use self::flow::FlowInterpreter;

mod flow;

// TODO: Give this a better name
pub enum FieldTypeResult {
    Normal(String),
    Embedded,
}
pub trait Interpreter {
    fn interpret(&self, ast: Vec<AST>) -> String;
    fn interpret_struct(&self, declaration: Box<StructDeclaration>) -> String;
    fn interpret_field(&self, field: Field) -> String;
    fn interpret_field_with_tags(
        &self,
        field_name: FieldName,
        field_type: FieldType,
        tags: HashMap<TagKey, TagValue>,
    ) -> String;
    fn convert_field_type(&self, field_type: FieldType) -> FieldTypeResult;
}

pub enum InterpreterImplementation {
    Flow,
}

pub trait IntepreterName {
    fn to_interpreter_name(&self) -> InterpreterImplementation;
}

impl IntepreterName for &str {
    fn to_interpreter_name(&self) -> InterpreterImplementation {
        let new_clone = self.to_ascii_lowercase();
        match new_clone.as_ref() {
            "flow" => InterpreterImplementation::Flow,
            _ => unimplemented!(),
        }
    }
}

impl IntepreterName for String {
    fn to_interpreter_name(&self) -> InterpreterImplementation {
        let new_clone = self.to_ascii_lowercase();
        match new_clone.as_ref() {
            "flow" => InterpreterImplementation::Flow,
            _ => unimplemented!(),
        }
    }
}

pub fn select_interpreter(interpreter: impl IntepreterName) -> impl Interpreter {
    let interpreter = interpreter.to_interpreter_name();
    match interpreter {
        InterpreterImplementation::Flow => FlowInterpreter::new(),
    }
}
