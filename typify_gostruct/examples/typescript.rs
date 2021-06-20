use typify_gostruct::Source;

fn main() -> Result<(), Vec<String>> {
    let example = r#"
    type Region struct {
    Country string `json:"country" binding:"required"`
    State string `json:"state" binding:"required"`
    }
    "#;

    let source = Source::new(example);
    let result = source.transform_to("typescript")?;
    println!("{}", result);
    Ok(())
}
