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
fn handles_nested_section_braces() {
	compare("[[a]]", "[[a]]\n");
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
	let utf8_bom: [u8; 3] = [0xef, 0xbb, 0xbf];
	let contents = format!("{}a=b", str::from_utf8(&utf8_bom).unwrap());
	compare(contents, "a=b\n");
}

#[test]
fn crlf_line_separators() {
	compare("[a]\r\nb=c", "[a]\nb=c\n");
}

/// Parse contents on the left and compare with expected output on the right.
fn compare<S: Into<String>>(contents: S, expected: &str) {
	let ast = parse(&contents.into()).unwrap();
	assert_eq!(ast.to_string(), expected);
}
