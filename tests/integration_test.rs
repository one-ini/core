//! Integration test suite for the parse function.

use editorconfig_ini::parse;
use std::str;

// Whitespace tests

#[test]
fn no_whitespace() {
	compare("a=b", "a=b\n");
}

#[test]
fn trims_spaces_around_equals() {
	compare("a = b", "a=b\n");
}

#[test]
fn trims_multiple_spaces_around_equals() {
	compare("a  =   b", "a=b\n");
}

#[test]
fn trims_spaces_before_pair_key() {
	compare("  a=b", "a=b\n");
}

#[test]
fn trims_spaces_after_pair_value() {
	compare("a=b  ", "a=b\n");
}

#[test]
fn removes_blank_lines_between_properties() {
	compare("\na=b\n\nc=d", "a=b\nc=d\n");
}

#[test]
fn includes_spaces_in_section_name() {
	compare("[ a b ]", "[ a b ]\n")
}

#[test]
fn trims_spaces_before_section_name() {
	compare("  [a]", "[a]\n");
}

#[test]
fn trims_spaces_after_section_name() {
	compare("[a]  ", "[a]\n");
}

#[test]
fn trims_spaces_before_middle_pair() {
	compare("a=b\n  c=d\ne=f", "a=b\nc=d\ne=f\n");
}

// Tests for comment parsing

#[test]
fn comment_indicator_in_section_before_pair() {
	compare("[a]\n;b\nc=d", "[a]\n; b\nc=d\n");
	compare("[a]\n#b\nc=d", "[a]\n# b\nc=d\n");
}

#[test]
fn comment_indicator_in_section_between_pairs() {
	compare("[a]\nb=c\n;d\ne=f", "[a]\nb=c\n; d\ne=f\n");
	compare("[a]\nb=c\n#d\ne=f", "[a]\nb=c\n# d\ne=f\n");
}

#[test]
fn comment_indicator_included_in_value() {
	compare("a=b; c", "a=b; c\n");
	compare("a=b# c", "a=b# c\n");
}

#[test]
fn escaped_comment_indicator_in_value() {
	compare("a=b\\;c", "a=b\\;c\n");
	compare("a=b\\#c", "a=b\\#c\n");
}

#[test]
fn escaped_comment_indicator_in_section_name() {
	compare("[a\\;b]", "[a\\;b]\n");
	compare("[a\\#b]", "[a\\#b]\n");
}

#[test]
fn removes_bom() {
	compare("\u{feff}a=b", "a=b\n");
}

#[test]
fn crlf_line_separators() {
	compare("[a]\r\nb=c", "[a]\nb=c\n");
}

// Test max property name and values

#[test]
fn max_pair_key_limit() {
	let limit = 50;
	let mut left = String::with_capacity(limit + 2);
	left.push_str(&"a".repeat(limit).to_string());
	left.push('=');
	let mut right = left.clone();
	right.push('\n');

	compare(&left, &right);

	left.pop();
	left.push_str("x=");

	match parse(&left) {
		Ok(_) => unreachable!(),
		Err(_) => (),
	}
}

#[test]
fn max_pair_value_limit() {
	let limit = 255;
	let mut contents = String::with_capacity(limit + 3);
	contents.push_str("a=");
	contents.push_str(&"b".repeat(limit).to_string());

	let mut right = contents.clone();
	right.push('\n');

	compare(&contents, &right);

	contents.push('b');

	match parse(&contents) {
		Ok(_) => unreachable!(),
		Err(_) => (),
	}
}

// #[test]
// fn max_section_name_limit() {
// 	let limit = 4096;
// 	let mut contents = String::with_capacity(limit + 3);
// 	contents.push('[');
// 	contents.push_str(&"a".repeat(limit).to_string());
// 	contents.push_str("]\n");

// 	compare(&contents, &contents);

// 	contents.pop();
// 	contents.pop();
// 	contents.push_str("a]\n");

// 	compare(&contents, &contents);

// 	// match parse(&contents) {
// 	// 	Ok(_) => unreachable!(),
// 	// 	Err(_) => (),
// 	// }
// }

/// Parse contents on the left and compare with expected output on the right.
fn compare<S: Into<String>>(contents: S, expected: &str) {
	let ast = parse(&contents.into()).unwrap();
	assert_eq!(ast.to_string(), expected);
}
