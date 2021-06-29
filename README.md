## Typify Gostruct

This project has 3 workspace projects,. (UI, <strike>coming soon</strike> is here)

1. typify_gostruct
2. wasm
3. www

### - typify_gostruct

This is the rust interpreter that parses & interprets the go-struct to interfaces of other languages.

Check out the dedicated readme of the interpreter [here](https://github.com/tevs-rust-land/typify_gostruct/tree/main/typify_gostruct)

### - wasm

This is the wasm wrapper of the interpreter, making it accessible to browser environments.

It exposes just 1 function `transform(source: str, to: str)`.
`Source` represents the struct in string format, and `To` is the name of the interpreter.

Link to npm package is -> https://www.npmjs.com/package/typify_gostruct_wasm

### - www

The UI simply imports the wasm wrapper & calls the exported wasm function. 
Give it a try here https://typify-gostruct.netlify.app/
![image](https://user-images.githubusercontent.com/12128153/123763721-bfef6f80-d8cc-11eb-96f5-3ae5601b8bcc.png)
