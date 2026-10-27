use one_ini::{OneINIAST, parse, parse_to_json, parse_to_uint32array};
use wasm_bindgen_test::wasm_bindgen_test;

#[wasm_bindgen_test]
fn token_offsets_match_the_native_contract() {
	assert_eq!(
		parse_to_uint32array(b"root=true\n[a]\nb=c").unwrap(),
		vec![0, 0, 4, 1, 5, 9, 2, 11, 12, 0, 14, 15, 1, 16, 17]
	);
}

#[wasm_bindgen_test]
fn invalid_utf8_returns_an_error() {
	assert!(parse_to_uint32array(&[0xff]).is_err());
}

#[wasm_bindgen_test]
fn javascript_ast_matches_the_native_ast() {
	let input = "root=true\n[ü]\n# comment\nname=🦀\n";
	let value = parse_to_json(input).unwrap();
	assert!(value.is_object());
	let ast: OneINIAST = serde_wasm_bindgen::from_value(value).unwrap();
	assert_eq!(ast, parse(input).unwrap());
}

#[wasm_bindgen_test]
fn empty_bodies_survive_javascript_serialization() {
	for input in ["", "[empty]\n"] {
		let ast: OneINIAST = serde_wasm_bindgen::from_value(parse_to_json(input).unwrap()).unwrap();
		assert_eq!(ast, parse(input).unwrap());
	}
}
