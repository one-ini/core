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
enum Item {
	Comment(Comment),
	Pair(Pair),
	Section(Section),
}

#[derive(Debug)]
pub struct EditorConfigINIAST {
	version: String,
	body: Vec<Item>,
}

#[derive(Debug)]
struct Section {
	name: String,
	body: Vec<Item>,
}

#[derive(Debug)]
struct Pair {
	key: String,
	value: String,
}

#[derive(Debug)]
struct Comment {
	indicator: String,
	value: String,
}
