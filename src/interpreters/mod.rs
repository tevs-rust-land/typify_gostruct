use crate::ast::AST;

use self::flow::FlowInterpreter;

mod flow;

pub enum FieldType {
    Normal(String),
    Embedded,
}
pub trait Interpreter {
    fn interpret(&self, ast: Vec<AST>) -> String;
}

pub enum InterpreterImplementation {
    Flow,
}

pub trait IntepreterName {
    fn to_interpreter_name(&self) -> InterpreterImplementation;
}

impl IntepreterName for &str {
    fn to_interpreter_name(&self) -> InterpreterImplementation {
        let name = self.to_ascii_lowercase();
        match name.as_ref() {
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
