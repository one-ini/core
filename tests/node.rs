//! Test suite for Node.js

#![cfg(test)]
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use std::assert_eq;

use one_ini::parse_to_uint32array;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn uint32array() {
	let expected: Vec<u32> = vec![0, 0, 4, 1, 5, 9, 2, 11, 12, 0, 14, 15, 1, 16, 17];
	match parse_to_uint32array(String::from("root=true\n[a]\nb=c").as_bytes()) {
		Ok(v) => assert_eq!(v, expected),
		Err(_) => assert!(false),
	}
}
