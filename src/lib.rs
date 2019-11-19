//! # EditorConfig
//!
//! `editorconfig` is a collection of utilities that handle the parsing of
//! EditorConfig-INI file contents into AST, modifying the AST and serializing
//! the result back into a string.

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
	match INIParser::parse(Rule::file, contents) {
		Ok(mut pairs) => {
			let item = pairs.next().unwrap();
			let mut current_section: Option<Section> = None;
			let mut ast = EditorConfigINIAST {
				version: "0.1.2".to_string(),
				body: Vec::new(),
			};

			for line in item.into_inner() {
				match line.as_rule() {
					Rule::section => {
						if current_section.is_some() {
							ast.body.push(Item::Section(current_section.unwrap()));
						}
						current_section = Some(Section {
							name: line.into_inner().next().unwrap().as_str().to_string(),
							body: Vec::new(),
						});
					}
					Rule::property => {
						let mut inner_rules = line.into_inner();
						let prop = Item::Property(Property {
							name: inner_rules.next().unwrap().as_str().to_string(),
							value: inner_rules.next().unwrap().as_str().to_string(),
							newline: "\n".to_string(),
						});
						match &mut current_section {
							Some(ref mut section) => {
								section.body.push(prop);
							}
							None => {
								ast.body.push(prop);
							}
						}
					}
					Rule::EOI => {
						if current_section.is_some() {
							ast.body.push(Item::Section(current_section.unwrap()));
							current_section = None;
						}
					}
					_ => unreachable!(),
				}
			}

			return Ok(ast);
		}
		Err(e) => Err(e),
	}
}

#[derive(Debug)]
enum Item {
	// BlankLine(BlankLine),
	// Comment(Comment),
	Property(Property),
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
struct Property {
	name: String,
	value: String,
	newline: String,
}

// #[derive(Debug)]
// struct BlankLine {
// 	newline: String,
// 	// raws.before
// }

// #[derive(Debug)]
// struct Comment {
// 	indicator: String,
// 	value: String,
// 	newline: String,
// 	// raws.before
// }
