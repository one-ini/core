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
use std::{env, fmt};

#[derive(Parser)]
#[grammar = "ini.pest"]
struct INIParser;

/// Parses [EditorConfig-INI](https://editorconfig-specification.readthedocs.io/en/latest/#file-format)
/// contents into [AST](https://en.wikipedia.org/wiki/Abstract_syntax_tree).
///
/// # Examples
///
/// ```
/// extern crate editorconfig;
///
/// let contents = "root=true";
/// let ast = editorconfig::parse(contents);
///
/// assert_eq!(ast.unwrap().to_string(), contents);
/// ```
pub fn parse(contents: &str) -> Result<EditorConfigINIAST, Error<Rule>> {
	match INIParser::parse(Rule::ini, contents) {
		Ok(mut pairs) => {
			return Ok(EditorConfigINIAST::new(create_body(pairs.next().unwrap())));
		}
		Err(e) => Err(e),
	}
}

fn create_body(pair: pest::iterators::Pair<'_, Rule>) -> Vec<Item> {
	return pair
		.into_inner()
		.map(|p| match p.as_rule() {
			Rule::section => {
				let mut inner_rules = p.into_inner();
				let name = inner_rules.next().unwrap().as_str().to_string();
				let body = inner_rules.next();
				return Item::Section(Section {
					name,
					body: if body.is_none() {
						vec![]
					} else {
						create_body(body.unwrap())
					},
				});
			}
			Rule::pair => {
				let mut inner_rules = p.into_inner();
				return Item::Pair(Pair {
					key: inner_rules.next().unwrap().as_str().to_string(),
					value: inner_rules.next().unwrap().as_str().to_string(),
				});
			}
			Rule::comment => {
				let mut inner_rules = p.into_inner();
				return Item::Comment(Comment {
					indicator: inner_rules.next().unwrap().as_str().to_string(),
					value: inner_rules.next().unwrap().as_str().to_string(),
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
/// # Examples
///
/// ```
/// extern crate editorconfig;
///
/// use editorconfig::{EditorConfigINIAST,Item,Pair};
///
/// let ast = EditorConfigINIAST::new(vec![
///     Item::Pair(Pair {
///         key: "root".to_string(),
///         value: "true".to_string(),
///     }),
/// ]);
///
/// assert_eq!(ast.to_string(), "root=true\n");
/// ```
#[derive(Debug)]
pub struct EditorConfigINIAST {
	version: String,
	pub body: Vec<Item>,
}

impl EditorConfigINIAST {
	fn new(body: Vec<Item>) -> Self {
		EditorConfigINIAST {
			version: env::var("version").unwrap(),
			body,
		}
	}
}

impl fmt::Display for EditorConfigINIAST {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		for item in &self.body {
			item.fmt(formatter)?;
		}
		Ok(())
	}
}

/// Any number of items may be used within a prelude or
/// [section](struct.section.html) body.
#[derive(Debug)]
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
/// extern crate editorconfig;
///
/// use editorconfig::Comment;
///
/// let comment = Comment {
///     indicator: "#".to_string(),
///     value: "hello",
/// };
///
/// assert_eq!(comment.to_string(), "# hello\n");
/// ```
#[derive(Debug)]
pub struct Comment {
	indicator: String,
	value: String,
}

impl fmt::Display for Comment {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		writeln!(formatter, "{} {}", self.indicator, self.value)?;
		Ok(())
	}
}

/// A key-value pair.
///
/// # Examples
///
/// ```
/// extern crate editorconfig;
///
/// use editorconfig::Pair;
///
/// let pair = Pair {
///     key: "left".to_string(),
///     value: "right".to_string(),
/// };
///
/// assert_eq!(pair.to_string(), "left=right\n");
/// ```
#[derive(Debug)]
pub struct Pair {
	key: String,
	value: String,
}

impl fmt::Display for Pair {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		writeln!(formatter, "{}={}", self.key, self.value)?;
		Ok(())
	}
}

/// Starts with a header and ends just before another section begins.
///
/// # Examples
///
/// ```
/// extern crate editorconfig;
///
/// use editorconfig::{Item,Section};
///
/// let section = Section {
///     name: "header".to_string(),
///     body: vec![
///         Item::Comment {
///             indicator: "#".to_string(),
///             value: "beginning of section body".to_string(),
///         },
///         Item::Pair {
///             key: "left".to_string(),
///             value: "right".to_string(),
///         },
///     ],
/// };
///
/// assert_eq!(section.to_string(), "[header]\nleft=right\n");
/// ```
#[derive(Debug)]
pub struct Section {
	/// The section header's name (i.e., the part between `[` and `]`).,
	name: String,
	/// Contains any number of items, which may only consist of
	/// comments and pairs.
	body: Vec<Item>,
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

	#[test]
	fn pair() {
		let pair = Pair {
			key: "left".to_string(),
			value: "right".to_string(),
		};

		assert_eq!(pair.key, "left");
		assert_eq!(pair.value, "right");
	}
}
