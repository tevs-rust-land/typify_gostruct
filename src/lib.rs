mod scanner;
mod treewalk;
use treewalk::interpreter;
use treewalk::parser;

pub fn transform(
    source: String,
    transform_to: interpreter::TransformTo,
) -> Result<String, Vec<treewalk::parser::ParseError>> {
    let (tokens, _scanner_errors) = scanner::scanner::scan(&source);
    match parser::parse(&tokens) {
        Ok(expr) => Ok(interpreter::interpret(&expr, transform_to)),
        Err(err) => Err(err),
    }
}
