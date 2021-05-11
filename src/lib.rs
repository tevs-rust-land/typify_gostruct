pub mod ast;
pub mod parser;
pub mod scanner;
// use treewalk::interpreter;
// fn transform(
//     source: String,
//     transform_to: interpreter::TransformTo,
// ) -> Result<String, Vec<String>> {
//     let (tokens, _scanner_errors) = scanner::scan(&source);
//     match parser::parse(&tokens) {
//         Ok(expr) => Ok(interpreter::interpret(&expr, transform_to)),
//         Err(err) => Err(err),
//     }
// }

// pub fn transform_to_flow(source: String) -> Result<String, Vec<String>> {
//     transform(source, interpreter::TransformTo::Flow)
// }

// pub fn transform_to_typescript(source: String) -> Result<String, Vec<String>> {
//     transform(source, interpreter::TransformTo::Typescript)
// }
