mod scanner;
mod treewalk;
use treewalk::interpreter;
use treewalk::parser;

pub fn run(source: String) -> String {
    let (tokens, _scanner_errors) = scanner::scanner::scan(&source);
    match parser::parse(&tokens) {
        Ok(expr) => interpreter::interpret(&expr),
        Err(err) => format!("error {:?}", err),
    }
}
