mod scanner;
mod treewalk;
use treewalk::interpreter;
use treewalk::parser;

fn transform(
    source: String,
    transform_to: interpreter::TransformTo,
) -> Result<String, Vec<treewalk::parser::ParseError>> {
    let (tokens, _scanner_errors) = scanner::scan(&source);
    match parser::parse(&tokens) {
        Ok(expr) => Ok(interpreter::interpret(&expr, transform_to)),
        Err(err) => Err(err),
    }
}

pub fn transform_to_flow(source: String) -> Result<String, Vec<treewalk::parser::ParseError>> {
    transform(source, interpreter::TransformTo::Flow)
}

pub fn transform_to_typescript(
    source: String,
) -> Result<String, Vec<treewalk::parser::ParseError>> {
    transform(source, interpreter::TransformTo::Typescript)
}
