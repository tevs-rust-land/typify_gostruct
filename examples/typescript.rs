fn main() -> Result<(), Vec<String>> {
    let example = r#"
    type Region struct {
    Country string `json:"country" binding:"required"`
    State string `json:"state" binding:"required"`
    }
    "#;

    let result = js_typify_gostruct::transform(example, "typescript")?;
    println!("{}", result);
    Ok(())
}
