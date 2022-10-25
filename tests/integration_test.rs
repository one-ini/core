//! Integration test suite for the parse function.

use one_ini::{parse, parse_to_vec, TokenTypes};
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
	compare("[a]\n;b\nc=d", "[a]\n;b\nc=d\n");
	compare("[a]\n#b\nc=d", "[a]\n#b\nc=d\n");
}

#[test]
fn comment_indicator_in_section_between_pairs() {
	compare("[a]\nb=c\n;d\ne=f", "[a]\nb=c\n;d\ne=f\n");
	compare("[a]\nb=c\n#d\ne=f", "[a]\nb=c\n#d\ne=f\n");
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

#[test]
fn no_whitespace_vec() {
	compare_vec(
		"a=b",
		&vec![(TokenTypes::Key, "a"), (TokenTypes::Value, "b")],
	);
}

#[test]
fn trims_spaces_around_equals_vec() {
	compare_vec(
		"a = b",
		&vec![(TokenTypes::Key, "a"), (TokenTypes::Value, "b")],
	);
}

#[test]
fn trims_multiple_spaces_around_equals_vec() {
	compare_vec(
		"a  =   b",
		&vec![(TokenTypes::Key, "a"), (TokenTypes::Value, "b")],
	);
}

#[test]
fn trims_spaces_before_pair_key_vec() {
	compare_vec(
		"  a=b",
		&vec![(TokenTypes::Key, "a"), (TokenTypes::Value, "b")],
	);
}

#[test]
fn trims_spaces_after_pair_value_vec() {
	compare_vec(
		"a=b  ",
		&vec![(TokenTypes::Key, "a"), (TokenTypes::Value, "b")],
	);
}

#[test]
fn removes_blank_lines_between_properties_vec() {
	compare_vec(
		"\na=b\n\nc=d",
		&vec![
			(TokenTypes::Key, "a"),
			(TokenTypes::Value, "b"),
			(TokenTypes::Key, "c"),
			(TokenTypes::Value, "d"),
		],
	);
}

#[test]
fn includes_spaces_in_section_name_vec() {
	compare_vec("[ a b ]", &vec![(TokenTypes::Section, " a b ")])
}

#[test]
fn trims_spaces_before_section_name_vec() {
	compare_vec("  [a]", &vec![(TokenTypes::Section, "a")]);
}

#[test]
fn trims_spaces_after_section_name_vec() {
	compare_vec("[a]  ", &vec![(TokenTypes::Section, "a")]);
}

#[test]
fn handles_nested_section_braces_vec() {
	compare_vec("[[a]]", &vec![(TokenTypes::Section, "[a]")]);
}

#[test]
fn trims_spaces_before_middle_pair_vec() {
	compare_vec(
		"a=b\n  c=d\ne=f",
		&vec![
			(TokenTypes::Key, "a"),
			(TokenTypes::Value, "b"),
			(TokenTypes::Key, "c"),
			(TokenTypes::Value, "d"),
			(TokenTypes::Key, "e"),
			(TokenTypes::Value, "f"),
		],
	);
}

// Tests for comment parsing to vec

#[test]
fn comment_indicator_in_section_before_pair_vec() {
	compare_vec(
		"[a]\n;b\nc=d",
		&vec![
			(TokenTypes::Section, "a"),
			(TokenTypes::CommentIndicator, ";"),
			(TokenTypes::CommentValue, "b"),
			(TokenTypes::Key, "c"),
			(TokenTypes::Value, "d"),
		],
	);
	compare_vec(
		"[a]\n#b\nc=d",
		&vec![
			(TokenTypes::Section, "a"),
			(TokenTypes::CommentIndicator, "#"),
			(TokenTypes::CommentValue, "b"),
			(TokenTypes::Key, "c"),
			(TokenTypes::Value, "d"),
		],
	);
}

#[test]
fn comment_indicator_in_section_between_pairs_vec() {
	compare_vec(
		"[a]\nb=c\n;d\ne=f",
		&vec![
			(TokenTypes::Section, "a"),
			(TokenTypes::Key, "b"),
			(TokenTypes::Value, "c"),
			(TokenTypes::CommentIndicator, ";"),
			(TokenTypes::CommentValue, "d"),
			(TokenTypes::Key, "e"),
			(TokenTypes::Value, "f"),
		],
	);
	compare_vec(
		"[a]\nb=c\n#d\ne=f",
		&vec![
			(TokenTypes::Section, "a"),
			(TokenTypes::Key, "b"),
			(TokenTypes::Value, "c"),
			(TokenTypes::CommentIndicator, "#"),
			(TokenTypes::CommentValue, "d"),
			(TokenTypes::Key, "e"),
			(TokenTypes::Value, "f"),
		],
	);
}

#[test]
fn comment_indicator_included_in_value_vec() {
	compare_vec(
		"a=b; c",
		&vec![(TokenTypes::Key, "a"), (TokenTypes::Value, "b; c")],
	);
	compare_vec(
		"a=b# c",
		&vec![(TokenTypes::Key, "a"), (TokenTypes::Value, "b# c")],
	);
}

#[test]
fn escaped_comment_indicator_in_value_vec() {
	// TODO: Not sure about this one.  Why and how does the test above
	// remove the backslash?
	compare_vec(
		"a=b\\;c",
		&vec![(TokenTypes::Key, "a"), (TokenTypes::Value, "b\\;c")],
	);
	compare_vec(
		"a=b\\#c",
		&vec![(TokenTypes::Key, "a"), (TokenTypes::Value, "b\\#c")],
	);
}

#[test]
fn escaped_comment_indicator_in_section_name_vec() {
	// TODO: Not sure about this one.  Why and how does the test above
	// remove the backslash?
	compare_vec("[a\\;b]", &vec![(TokenTypes::Section, "a\\;b")]);
	compare_vec("[a\\#b]", &vec![(TokenTypes::Section, "a\\#b")]);
}

#[test]
fn removes_bom_vec() {
	compare_vec(
		"\u{feff}a=b",
		&vec![(TokenTypes::Key, "a"), (TokenTypes::Value, "b")],
	);
}

#[test]
fn crlf_line_separators_vec() {
	compare_vec(
		"[a]\r\nb=c",
		&vec![
			(TokenTypes::Section, "a"),
			(TokenTypes::Key, "b"),
			(TokenTypes::Value, "c"),
		],
	);
}

#[test]
fn partial_section_vec() {
	compare_vec("[foo", &vec![]);
}

// Tests for the test harness
#[test]
#[should_panic]
fn compare_vec_not_enough() {
	compare_vec("", &vec![(TokenTypes::Key, "a")]);
}

#[test]
#[should_panic]
fn compare_vec_too_many() {
	compare_vec("a=b", &vec![(TokenTypes::Key, "a")]);
}

#[test]
#[should_panic]
fn compare_vec_wrong_token() {
	compare_vec("a=b", &vec![(TokenTypes::Key, "a"), (TokenTypes::Key, "b")]);
}

#[test]
#[should_panic]
fn compare_vec_wrong_value() {
	compare_vec(
		"a=b",
		&vec![(TokenTypes::Key, "a"), (TokenTypes::Value, "c")],
	);
}

fn compare_vec<S: Into<String>>(contents: S, expected: &Vec<(TokenTypes, &str)>) {
	let s: String = contents.into();
	let v = parse_to_vec(&s).unwrap();
	let buf = s.as_bytes();

	let mut i = 0;
	for chunk in v.chunks(3) {
		let (etyp, estr) = expected[i];
		assert_eq!(chunk[0], etyp as u32);
		assert_eq!(
			str::from_utf8(&buf[chunk[1] as usize..chunk[2] as usize]).unwrap(),
			estr
		);
		i += 1;
	}
	assert_eq!(i, expected.len());
}
