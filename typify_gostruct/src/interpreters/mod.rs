use crate::ast::AST;
use std::fmt::{self, Display};

use self::{flow::FlowInterpreter, typescript::TypeScriptInterpreter};

mod flow;
mod typescript;

pub enum FieldType {
    Normal(String),
    Embedded,
}

#[derive(PartialEq, Debug)]
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

pub trait ToInterpreter {
    fn convert(&self) -> Result<Box<dyn Interpreter>, InterpreterError>;
}

macro_rules! interpreter_impl_for {
    ($t:ty) => {
        impl ToInterpreter for $t {
            fn convert(&self) -> Result<Box<dyn Interpreter>, InterpreterError> {
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

#[cfg(test)]
mod tests {
    use crate::interpreters::InterpreterError;

    use super::ToInterpreter;

    #[test]
    fn invalid_interpreter_name_should_return_error() {
        let target = "glow".to_string();
        let result = target.convert();
        match result {
            Ok(_interpreter) => panic!("Should not return an interpreter"),
            Err(err) => {
                assert_eq!(
                    err,
                    InterpreterError::UnexpectedInterpreterName("glow".to_string())
                )
            }
        }
    }

    #[test]
    fn test_should_return_valid_interpreter() {
        let target = "flow".to_string();
        let result = target.convert();
        match result {
            Ok(interpreter) => {
                let result = interpreter
                    .interpret(Vec::new())
                    .expect("Should not fail to interpret");
                assert!(result.contains("// @flow"))
            }
            Err(_err) => {
                panic!("Flow is a valid interpreter")
            }
        }
    }
}
