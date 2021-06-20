mod utils;

use typify_gostruct::Source;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Returns the transformed go-lang struct.
/// # Arguments
///
/// * `source` - A string representation of the go struct to be transfomed.
///
/// * `to` - To represents the target interpreter that will be used for the transformation. Can either be "flow" | "typescript"
///
///
/// * #### N/B: The targets that can be used as of now are, "flow" & "typescript" (more to come...).
#[wasm_bindgen]
pub fn transform(source: &str, to: &str) -> std::result::Result<String, JsValue> {
    let source = Source::new(source);
    source
        .transform_to(to)
        .map_err(|err| JsValue::from_str(&err.join("")))
}
