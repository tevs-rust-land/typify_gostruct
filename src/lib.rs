use interpreters::{select_interpreter, TargetIntepreter};
use scanner::Input;

pub mod ast;
pub mod interpreters;
pub mod parser;
pub mod scanner;

pub struct Source<S>(S);

impl<S> Source<S>
where
    S: Input + Copy,
{
    pub fn new(source: S) -> Self {
        Self(source)
    }

    pub fn transform_to<T>(&self, target: T) -> Result<String, Vec<String>>
    where
        T: TargetIntepreter,
    {
        let tokens = scanner::scan(self.0)?;
        let parsed_result = parser::parse(&tokens)?;
        let interpreter = select_interpreter(target)?;
        interpreter
            .interpret(parsed_result)
            .map_err(|err| err.into())
    }
}
