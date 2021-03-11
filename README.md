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
    // converts to flow
    match js_typify_gostruct::transform_to_flow(example.to_string()) {
        Ok(results) => println!("{}", results), // prints out export type Region = { country:string; state:string; };
        Err(parse_errors) => println!("{:?}", parse_errors),
    }

    // converts to typescript
    match js_typify_gostruct::transform_to_typescript(example.to_string()) {
        Ok(results) => println!("{}", results), // prints out export interface Region { country:string; state:string; };
        Err(parse_errors) => println!("{:?}", parse_errors),
    }
}

```

### Running the examples

The examples folder contains various examples of how the library works & is to be used.

To run a specific example run the following command

eg

```
cargo run --example flow
```
