fn main() -> Result<(), Vec<String>> {
    let example = r#"
    type Region struct {
    Country string `json:"country" binding:"required"`
    State string 
    }
    "#;

    let result = js_typify_gostruct::transform(example, "flow")?;
    println!("{}", result);
    Ok(())
}
