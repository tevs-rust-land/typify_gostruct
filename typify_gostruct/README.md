[![Gitpod ready-to-code](https://img.shields.io/badge/Gitpod-ready--to--code-blue?logo=gitpod)](https://gitpod.io/#https://github.com/tevs-rust-land/js_typify_gostruct)

## JS_TYPIFY_GOSTRUCT

A rust tool meant to convert a golang struct to a js type object.

Based on the new re-write the name of this library might change,
Its now possible to have interpreters for all sorts of languages, not just Javascript based Languages.

## How to use

```rs
use js_typify_gostruct::Source;

fn main() -> Result<(), Vec<String>> {
    let example = r#"
    type Region struct {
    Country string `json:"country"`
    State string `json:"state"`
}
    "#;
    // converts to flow

    let source = Source::new(example);
    let result = source.transform_to("flow")?;
    println!("{}", result);
    // result will be
    // // @flow
   //export type Region = {country : string, state : string, }

    // converts to typescript
    let result = source.transform_to("typescript")?;
    println!("{}", result);

    // result will be
   //export interface Region = {country : string, state : string, }

   Ok(())
}

```

### Running the examples

The examples folder contains various examples of how the library works & is to be used.

To run a specific example run the following command

eg

```
cargo run --example flow
```

```
cargo run --example typescript
```

TODO

- [x] Re-introduce tests & fix bugs.
- [x] Add documentation for the library.
- [ ] Introduce interpreter support for other languages,eg, gostruct -> rust struct....
- [x] Make a WASM wrapper for this library so that it can be used natively on the web.
