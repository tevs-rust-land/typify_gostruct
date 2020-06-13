## JS_TYPIFY_GOSTRUCT

A rust tool meant to convert a golang struct to a js type object.

## How to use

```rs
use js_typify_gostruct;

fn main() {
    let example = r#"
    type Region struct {
    Country string `json:"country" binding:"required"`
    State string `json:"state" binding:"required"`
}
    "#;
    match js_typify_gostruct::transform(example.to_string()) {
        Ok(results) => println!("{}", results); // prints out type Region = { country:string; state:string; };
        Err(parse_errors) => println!("{:?}", parse_errors);
    }
}

```
