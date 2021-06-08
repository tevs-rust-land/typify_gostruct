use interpreters::{select_interpreter, TargetIntepreter};

pub mod ast;
pub mod interpreters;
pub mod parser;
pub mod scanner;

pub trait Source {
    fn to(&self) -> &str;
}

impl Source for &str {
    fn to(&self) -> &str {
        self
    }
}

impl Source for String {
    fn to(&self) -> &str {
        self.as_str()
    }
}

pub fn transform<S, T>(source: S, target: T) -> Result<String, Vec<String>>
where
    S: Source,
    T: TargetIntepreter,
{
    let source = source.to();
    let tokens = scanner::scan(source)?;
    let parsed_result = parser::parse(&tokens)?;
    let interpreter = select_interpreter(target);
    Ok(interpreter.interpret(parsed_result))
}
