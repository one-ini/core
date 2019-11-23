//! # EditorConfig
//!
//! `editorconfig` is a collection of utilities that handle the parsing of
//! EditorConfig-INI file contents into AST, which can then be modified,
//! serialized and deserialized.

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::error::Error;
use pest::Parser;
use std::fmt;

#[derive(Parser)]
#[grammar = "ini.pest"]
pub struct INIParser;

/// Parses EditorConfig-INI contents into AST.
///
/// # Examples
///
/// ```
/// let contents = "root=true";
/// let ast = editorconfig::parse(contents);
/// ```
pub fn parse(contents: &str) -> Result<EditorConfigINIAST, Error<Rule>> {
	match INIParser::parse(Rule::ini, contents) {
		Ok(mut pairs) => {
			return Ok(EditorConfigINIAST {
				version: "0.1.0".to_string(),
				body: create_body(pairs.next().unwrap()),
			});
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

#[derive(Debug)]
pub struct EditorConfigINIAST {
	pub version: String,
	pub body: Vec<Item>,
}

impl fmt::Display for EditorConfigINIAST {
	fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
		for item in &self.body {
			item.fmt(formatter)?;
		}
		Ok(())
	}
}

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

#[derive(Debug)]
pub struct Section {
	name: String,
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
