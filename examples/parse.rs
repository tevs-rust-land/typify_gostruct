fn main() {
    let example = r#"
    type Region struct {
    Country string
    State string
    }
    "#;
    let (tokens, _error) = js_typify_gostruct::scanner::scan(example);
    let scann = js_typify_gostruct::parser::parse(&tokens).map_err(|_err| ());
    println!("{:?}", scann);
}
