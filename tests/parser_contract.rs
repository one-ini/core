use one_ini::{Item, parse, parse_to_vec};

#[test]
fn unicode_offsets_are_utf8_byte_offsets() {
	let input = "\u{feff}[ü]\r\n名=🦀\r\n";
	let tokens = parse_to_vec(input).unwrap();
	assert_eq!(tokens, vec![2, 4, 6, 0, 9, 12, 1, 13, 17]);
	for token in tokens.chunks_exact(3) {
		assert!(input.is_char_boundary(token[1] as usize));
		assert!(input.is_char_boundary(token[2] as usize));
	}
}

#[test]
fn duplicate_keys_and_sections_keep_their_order() {
	let input = "a=1\na=2\n\n[x]\nb=3\n\n[x]\nb=4\n";
	let ast = parse(input).unwrap();
	assert_eq!(ast.body.len(), 4);
	assert!(matches!(&ast.body[0], Item::Pair(pair) if pair.value == "1"));
	assert!(matches!(&ast.body[1], Item::Pair(pair) if pair.value == "2"));
	assert_eq!(ast.to_string(), input);
}

#[test]
fn incomplete_suffix_keeps_the_valid_prefix() {
	// Existing tolerant behavior; strict parsing needs a separate API decision.
	assert_eq!(parse("a=b\n[unfinished").unwrap().to_string(), "a=b\n");
	assert_eq!(
		parse_to_vec("a=b\n[unfinished").unwrap(),
		vec![0, 0, 1, 1, 2, 3]
	);
}
