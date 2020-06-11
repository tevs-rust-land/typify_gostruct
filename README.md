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
    println!("{:?}", js_typify_gostruct::run(example.to_string())); // prints out type Region = { country:string; state:string; };
}

```
