//! Binding contracts in a real browser.
#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::wasm_bindgen_test_configure;

wasm_bindgen_test_configure!(run_in_browser);

#[path = "common/wasm.rs"]
mod contracts;
