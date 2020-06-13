mod scanner;
mod treewalk;
use treewalk::interpreter;
use treewalk::parser;

pub fn transform(source: String) -> Result<String, Vec<treewalk::parser::ParseError>> {
    let (tokens, _scanner_errors) = scanner::scanner::scan(&source);
    match parser::parse(&tokens) {
        Ok(expr) => Ok(interpreter::interpret(&expr)),
        Err(err) => Err(err),
    }
}
