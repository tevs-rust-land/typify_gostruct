fn main() {
    println!(" ---This is the output of a valid struct ---");
    valid_struct_definition();
    println!(" ---This is the output of an invalid struct ---");

    invalid_struct_definition()
}

fn valid_struct_definition() {
    let example = r#"
    type Region struct {
    Country string `json:"country" binding:"required"`
    State string `json:"state" binding:"required"`
}
    "#;
    // converts to typescript
    match js_typify_gostruct::transform_to_typescript(example.to_string()) {
        Ok(results) => println!("{}", results),
        Err(parse_errors) => println!("{:?}", parse_errors),
    }
}

fn invalid_struct_definition() {
    let example = r#"
    type Region struct
    Country string json:"country" binding:"required"
    State string `json:"state" binding:"required"`
}
    "#;
    // converts to typescript
    match js_typify_gostruct::transform_to_typescript(example.to_string()) {
        Ok(results) => println!("{}", results),
        Err(parse_errors) => println!("{:?}", parse_errors),
    }
}
