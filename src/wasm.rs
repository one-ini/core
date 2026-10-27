//! JavaScript bindings for the portable Rust parser.

use wasm_bindgen::prelude::*;

use crate::{parse, parse_to_vec};

/// Returns the AST as a JavaScript object (despite the historical name).
/// Parser and serialization failures are thrown as JavaScript errors.
#[wasm_bindgen]
pub fn parse_to_json(contents: &str) -> Result<JsValue, JsError> {
	crate::utils::set_panic_hook();
	let ast = parse(contents)?;
	Ok(serde_wasm_bindgen::to_value(&ast)?)
}

/// Returns token types and UTF-8 byte offsets as a JavaScript Uint32Array.
#[wasm_bindgen]
pub fn parse_to_uint32array(contents: &[u8]) -> Result<Vec<u32>, JsError> {
	crate::utils::set_panic_hook();
	let input = std::str::from_utf8(contents)?;
	Ok(parse_to_vec(input)?)
}
