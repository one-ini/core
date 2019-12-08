//! # EditorConfig
//!
//! A collection of utilities that handle the parsing of
//! [EditorConfig-INI](https://editorconfig-specification.readthedocs.io/en/latest/#file-format)
//! file contents into [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree),
//! which can then be modified and/or serialized.

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::Parser;
use serde::{Deserialize, Serialize};
use std::{env, fmt, str};
use wasm_bindgen::prelude::*;

#[derive(Parser)]
#[grammar = "ini.pest"]
struct INIParser;

#[wasm_bindgen]
pub fn parse_to_json(contents: &str) -> JsValue {
	let ast = parse(&contents).unwrap();
	return JsValue::from_serde(&ast).unwrap();
}

/// Parses [EditorConfig-INI](https://editorconfig-specification.readthedocs.io/en/latest/#file-format)
/// contents into [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree).
///
/// # Example
///
/// ```
/// let contents = String::from("root=true\n");
/// let ast = editorconfig_ini::parse(&contents).unwrap();
///
/// assert_eq!(ast.to_string(), contents);
/// ```
pub fn parse(contents: &str) -> Result<EditorConfigINIAST, Error<Rule>> {
	return match INIParser::parse(Rule::ini, contents) {
		Ok(mut pairs) => Ok(EditorConfigINIAST::new(create_body(pairs.next().unwrap()))),
		Err(e) => Err(e),
	};
}

fn create_body(pair: pest::iterators::Pair<'_, Rule>) -> Vec<Item> {
	return pair
		.into_inner()
		.filter(|p| match p.as_rule() {
			Rule::EOI => false,
			_ => true,
		})
		.map(|p| match p.as_rule() {
			Rule::section => {
				let mut inner_rules = p.into_inner();
				let header = String::from(inner_rules.next().unwrap().as_str());
				return Item::Section(Section {
					name: header[1..(header.len() - 1)].to_string(),
					body: match inner_rules.next() {
						Some(pair) => create_body(pair),
						_ => vec![],
					},
				});
			}
			Rule::pair => {
				let mut inner_rules = p.into_inner();
				return Item::Pair(Pair {
					key: String::from(inner_rules.next().unwrap().as_str()),
					value: String::from(inner_rules.next().unwrap().as_str()),
				});
			}
			Rule::comment => {
				let mut inner_rules = p.into_inner();
				return Item::Comment(Comment {
					indicator: inner_rules.next().unwrap().as_str().chars().nth(0).unwrap(),
					value: String::from(inner_rules.next().unwrap().as_str()),
				});
			}
			_ => unreachable!(),
		})
		.collect();
}

/// The root [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree) node of
/// a [parsed](fn.parse.html) INI file that conforms to the
/// [EditorConfig INI file format](https://editorconfig-specification.readthedocs.io/en/latest/#file-format).
///
/// # Example
///
/// ```
/// use editorconfig_ini::*;
///
/// let ast = EditorConfigINIAST::new(vec![
///     Item::Pair(Pair {
///         key: String::from("root"),
///         value: String::from("true"),
///     }),
///     Item::Section(Section {
///         name: String::from("one"),
///         body: vec![
///             Item::Comment(Comment {
///                 indicator: '#',
///                 value: String::from("body1"),
///             }),
///         ],
///     }),
///     Item::Section(Section {
///         name: String::from("two"),
///         body: vec![
///             Item::Comment(Comment {
///                 indicator: ';',
///                 value: String::from("body2"),
///             }),
///         ],
///     }),
/// ]);
///
/// assert_eq!(ast.to_string(), "root=true\n\n[one]\n# body1\n\n[two]\n; body2\n");
///
/// let serialized = serde_json::to_string(&ast).unwrap();
/// let expected = "{\"version\":\"0.1.0\",\"body\":[{\"type\":\"Pair\",\"key\":\"root\",\"value\":\"true\"},{\"type\":\"Section\",\"name\":\"one\",\"body\":[{\"type\":\"Comment\",\"indicator\":\"#\",\"value\":\"body1\"}]},{\"type\":\"Section\",\"name\":\"two\",\"body\":[{\"type\":\"Comment\",\"indicator\":\";\",\"value\":\"body2\"}]}]}";
/// assert_eq!(serialized, expected);
///
/// let deserialized: EditorConfigINIAST = serde_json::from_str(&serialized).unwrap();
/// assert_eq!(serde_json::to_string(&deserialized).unwrap(), expected);
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct EditorConfigINIAST {
	/// The version of the EditorConfig-INI parser.
	pub version: String,
	/// Contains the _prelude_, followed by any number of sections.
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub body: Vec<Item>,
}

impl EditorConfigINIAST {
	pub fn new<B: Into<Vec<Item>>>(body: B) -> Self {
		EditorConfigINIAST {
			version: String::from(env!("CARGO_PKG_VERSION")),
			body: body.into(),
		}
	}
}

impl fmt::Display for EditorConfigINIAST {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		let mut wrote = false;
		for item in &self.body {
			match item {
				Item::Section(_section) => {
					if wrote {
						writeln!(formatter)?;
					}
				}
				_ => (),
			}
			item.fmt(formatter)?;
			wrote = true;
		}
		Ok(())
	}
}

/// Any number of items may be used within a prelude or
/// [section](struct.section.html) body.
///
/// # Serializing & Deserializing
///
/// ```
/// use editorconfig_ini::{Comment,Item};
///
/// let item = Item::Comment(Comment {
///     indicator: '#',
///     value: String::from("octothorpe"),
/// });
/// let serialized = serde_json::to_string(&item).unwrap();
/// assert_eq!(
///     serialized,
///     "{\"type\":\"Comment\",\"indicator\":\"#\",\"value\":\"octothorpe\"}",
/// );
///
/// let deserialized: Comment = serde_json::from_str(&serialized).unwrap();
/// assert_eq!(deserialized.indicator, '#');
/// assert_eq!(deserialized.value, "octothorpe");
/// ```
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Item {
	Comment(Comment),
	Pair(Pair),
	Section(Section),
}

impl fmt::Display for Item {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Item::Comment(comment) => comment.fmt(formatter),
			Item::Pair(pair) => pair.fmt(formatter),
			Item::Section(section) => section.fmt(formatter),
		}?;
		Ok(())
	}
}

/// Starts with either a `#` or `;` comment indicator on a new or blank line,
/// followed by any characters until it reaches a newline or the end of input.
///
/// # Examples
///
/// ```
/// let comment = editorconfig_ini::Comment {
///     indicator: '#',
///     value: String::from("octothorpe"),
/// };
///
/// assert_eq!(comment.to_string(), "# octothorpe\n");
/// ```
///
/// ```
/// let comment = editorconfig_ini::Comment {
///     indicator: ';',
///     value: String::from("semi-colon"),
/// };
///
/// assert_eq!(comment.to_string(), "; semi-colon\n");
/// ```
///
/// # Serializing & Deserializing
///
/// ```
/// let comment = editorconfig_ini::Comment {
///     indicator: '#',
///     value: String::from("octothorpe"),
/// };
/// let serialized = serde_json::to_string(&comment).unwrap();
/// let deserialized: editorconfig_ini::Comment = serde_json::from_str(&serialized).unwrap();
///
/// assert_eq!(
///     serialized,
///     "{\"indicator\":\"#\",\"value\":\"octothorpe\"}",
/// );
/// assert_eq!(deserialized.indicator, '#');
/// assert_eq!(deserialized.value, "octothorpe");
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Comment {
	/// The character that begins a comment. This may only be
	/// an octothorpe (`#`) or a semi-colon (`;`).
	pub indicator: char,
	/// The value that follows the comment indicator.
	pub value: String,
}

/// Serializes a comment as a JSON string.
///
/// # Example
///
/// ```
/// let comment = editorconfig_ini::Comment {
///     indicator: '#',
///     value: String::from("octothorpe"),
/// };
/// let serialized = serde_json::to_string(&comment).unwrap();
/// let deserialized: editorconfig_ini::Comment = serde_json::from_str(&serialized).unwrap();
///
/// assert_eq!(
///     serialized,
///     "{\"indicator\":\"#\",\"value\":\"octothorpe\"}",
/// );
/// assert_eq!(deserialized.indicator, '#');
/// assert_eq!(deserialized.value, "octothorpe");
/// ```
// impl Serialize for Comment {
// 	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
// 	where
// 		S: Serializer,
// 	{
// 		let mut state = serializer.serialize_struct("Comment", 3)?;
// 		// state.serialize_field("type", "comment")?;
// 		state.serialize_field("indicator", &self.indicator)?;
// 		state.serialize_field("value", &self.value)?;
// 		return state.end();
// 	}
// }

impl fmt::Display for Comment {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		writeln!(formatter, "{} {}", self.indicator, self.value)?;
		Ok(())
	}
}

/// A key-value pair.
///
/// # Example
///
/// ```
/// let pair = editorconfig_ini::Pair {
///     key: String::from("left"),
///     value: String::from("right"),
/// };
///
/// assert_eq!(pair.to_string(), "left=right\n");
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Pair {
	/// Appears on the _left_ side of the assignment (`=`).
	pub key: String,
	/// Appears on the _right_ side of the assignment (`=`).
	pub value: String,
}

impl fmt::Display for Pair {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		writeln!(formatter, "{}={}", self.key.trim(), self.value.trim())?;
		Ok(())
	}
}

/// Starts with a header and ends just before another section begins.
///
/// # Example
///
/// ```
/// use editorconfig_ini::*;
///
/// let section = Section {
///     name: String::from("header"),
///     body: vec![
///         Item::Comment(Comment {
///             indicator: '#',
///             value: String::from("body"),
///         }),
///         Item::Pair(Pair {
///             key: String::from("left"),
///             value: String::from("right"),
///         }),
///     ],
/// };
///
/// assert_eq!(section.to_string(), "[header]\n# body\nleft=right\n");
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Section {
	/// The section header's name (i.e., the part between `[` and `]`).,
	pub name: String,
	/// Contains any number of items, which may only consist of
	/// comments and pairs.
	#[serde(skip_serializing_if = "Vec::is_empty")]
	pub body: Vec<Item>,
}

impl fmt::Display for Section {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		writeln!(formatter, "[{}]", self.name)?;
		for item in &self.body {
			item.fmt(formatter)?;
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::fs;

	#[test]
	fn it_works() {
		let contents = fs::read_to_string("tests/fixtures/config.ini").unwrap();
		let ast = parse(&contents).unwrap();
		assert_eq!(ast.to_string(), contents);
	}
}
