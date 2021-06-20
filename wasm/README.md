## typify_gostruct_wasm

A wrapper around the rust gostruct type interpreter. -> https://github.com/tevs-rust-land/typify_gostruct

## About

The struct interpreter converts a struct to type objects/interfaces of provided languages.

The current languages available are `flow` & `typescript`. I'll add support for other languages as time goes.

## 🚴 Usage

```js
import("typify_gostruct_wasm").then(({ transform }) => {
  const struct = `
      type person struct {
        name string
        age  int
    }
      `;

  try {
    const flowResult = transform(struct, "flow");
    const typescriptResult = transform(struct, "typescript");
  } catch (err) {}
});
```

This wasm module has been Built with 🦀 & 🕸 assembly.

## 🔋 Batteries Included

- [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) for communicating
  between WebAssembly and JavaScript.
- [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook)
  for logging panic messages to the developer console.
- [`wee_alloc`](https://github.com/rustwasm/wee_alloc), an allocator optimized
  for small code size.
