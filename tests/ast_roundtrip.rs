use one_ini::{OneINIAST, parse};

#[test]
fn empty_document_survives_json_roundtrip() {
	let ast = parse("").unwrap();
	let json = serde_json::to_string(&ast).unwrap();
	let restored: OneINIAST = serde_json::from_str(&json).unwrap();
	assert_eq!(restored, ast);
	assert!(restored.body.is_empty());
}

#[test]
fn empty_section_survives_json_roundtrip() {
	let ast = parse("[empty]\n").unwrap();
	let json = serde_json::to_string(&ast).unwrap();
	let restored: OneINIAST = serde_json::from_str(&json).unwrap();
	assert_eq!(restored, ast);
}
