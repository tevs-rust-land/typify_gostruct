use crate::ast::AST;
use std::fmt::{self, Display};

use self::{flow::FlowInterpreter, typescript::TypeScriptInterpreter};

mod flow;
mod typescript;

pub enum FieldType {
    Normal(String),
    Embedded,
}

pub enum InterpreterError {
    ExpectedStructFoundField,
    UnexpectedInterpreterName(String),
}

impl Display for InterpreterError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            InterpreterError::ExpectedStructFoundField => {
                write!(f, "Expected a struct but found a field")
            }
            InterpreterError::UnexpectedInterpreterName(namegiven) => {
                write!(
                    f,
                    "You have provided {}. Please provide either `flow` | `typescript` ",
                    namegiven
                )
            }
        }
    }
}

impl From<InterpreterError> for Vec<String> {
    fn from(error: InterpreterError) -> Self {
        vec![format!("{}", error)]
    }
}

pub trait Interpreter {
    fn interpret(&self, ast: Vec<AST>) -> Result<String, InterpreterError>;
}

pub trait TargetIntepreter {
    fn get_implementation(&self) -> Result<Box<dyn Interpreter>, InterpreterError>;
}

macro_rules! interpreter_impl_for {
    ($t:ty) => {
        impl TargetIntepreter for $t {
            fn get_implementation(&self) -> Result<Box<dyn Interpreter>, InterpreterError> {
                let name = self.to_ascii_lowercase();
                match name.as_ref() {
                    "flow" => Ok(Box::new(FlowInterpreter::new())),
                    "typescript" => Ok(Box::new(TypeScriptInterpreter::new())),
                    _ => return Err(InterpreterError::UnexpectedInterpreterName(name)),
                }
            }
        }
    };
}

interpreter_impl_for!(&str);
interpreter_impl_for!(String);

pub fn select_interpreter(
    interpreter: impl TargetIntepreter,
) -> Result<Box<dyn Interpreter>, InterpreterError> {
    interpreter.get_implementation()
}
