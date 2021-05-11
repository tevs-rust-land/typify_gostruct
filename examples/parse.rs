use js_typify_gostruct;
fn main() -> () {
    let example = r#"
    type Region struct {
    Country string
    State string
    }
    "#;
    let (tokens, _error) = js_typify_gostruct::scanner::scan(example);
    let scann = js_typify_gostruct::treewalk::new_parser::parse(&tokens).map_err(|err| ());
    println!("{:?}", scann);

}
