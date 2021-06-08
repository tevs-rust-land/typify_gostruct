use crate::ast::AST;

use self::{flow::FlowInterpreter, typescript::TypeScriptInterpreter};

mod flow;
mod typescript;

pub enum FieldType {
    Normal(String),
    Embedded,
}
pub trait Interpreter {
    fn interpret(&self, ast: Vec<AST>) -> String;
}

pub trait TargetIntepreter {
    fn get_implementation(&self) -> Box<dyn Interpreter>;
}

impl TargetIntepreter for &str {
    fn get_implementation(&self) -> Box<dyn Interpreter> {
        let name = self.to_ascii_lowercase();
        match name.as_ref() {
            "flow" => Box::new(FlowInterpreter::new()),
            "typescript" => Box::new(TypeScriptInterpreter::new()),
            _ => unimplemented!(),
        }
    }
}

impl TargetIntepreter for String {
    fn get_implementation(&self) -> Box<dyn Interpreter> {
        let new_clone = self.to_ascii_lowercase();
        match new_clone.as_ref() {
            "flow" => Box::new(FlowInterpreter::new()),
            "typescript" => Box::new(TypeScriptInterpreter::new()),
            _ => unimplemented!(),
        }
    }
}

pub fn select_interpreter(interpreter: impl TargetIntepreter) -> Box<dyn Interpreter> {
    interpreter.get_implementation()
}
