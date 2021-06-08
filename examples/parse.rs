fn main() {
    let example = r#"
    type Region struct {
    Country string `json:"country" binding:"required"`
    State string 
    }
    "#;
    let (tokens, _error) = js_typify_gostruct::scanner::scan(example);
    println!("{:?}", tokens);
    let scann = js_typify_gostruct::parser::parse(&tokens);
    println!("{:?}", scann);
}
