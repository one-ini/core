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
	match INIParser::parse(Rule::ini, contents) {
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
							header: SectionHeader {
								name: line.into_inner().next().unwrap().as_str().to_string(),
								raws: Raws {
									before: Some("".to_string()),
									after: Some("".to_string()),
									newline: Some("\n".to_string()),
								},
							},
							body: Vec::new(),
						});
					}
					Rule::pair => {
						let mut inner_rules = line.into_inner();
						let prop = Item::Pair(Pair {
							name: inner_rules.next().unwrap().as_str().to_string(),
							value: inner_rules.next().unwrap().as_str().to_string(),
							raws: Raws {
								before: Some("".to_string()),
								after: Some("".to_string()),
								newline: Some("\n".to_string()),
							},
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
					Rule::comment => {
						let mut inner_rules = line.into_inner();
						let comment = Item::Comment(Comment {
							indicator: inner_rules.next().unwrap().as_str().to_string(),
							value: inner_rules.next().unwrap().as_str().to_string(),
							raws: Raws {
								before: Some("".to_string()),
								after: Some("".to_string()),
								newline: Some(inner_rules.next().unwrap().as_str().to_string()),
							},
						});
						match &mut current_section {
							Some(ref mut section) => {
								section.body.push(comment);
							}
							None => {
								ast.body.push(comment);
							}
						}
					}
					Rule::blank_line => {
						let mut inner_rules = line.into_inner();
						let whitespace = inner_rules.next().unwrap().as_str().to_string();
						let line = Item::Raws(Raws {
							before: if whitespace == "" {
								Some(whitespace)
							} else {
								None
							},
							after: None,
							newline: Some(inner_rules.next().unwrap().as_str().to_string()),
						});
						match &mut current_section {
							Some(ref mut section) => {
								section.body.push(line);
							}
							None => {
								ast.body.push(line);
							}
						}
					}
					Rule::ws => {
						let line = Item::Raws(Raws {
							before: Some(line.into_inner().next().unwrap().as_str().to_string()),
							after: None,
							newline: None,
						});
						match &mut current_section {
							Some(ref mut section) => {
								section.body.push(line);
							}
							None => {
								ast.body.push(line);
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
	Raws(Raws),
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
	header: SectionHeader,
	body: Vec<Item>,
}

#[derive(Debug)]
struct SectionHeader {
	name: String,
	raws: Raws,
}

#[derive(Debug)]
struct Raws {
	before: Option<String>,
	after: Option<String>,
	newline: Option<String>,
}

#[derive(Debug)]
struct Pair {
	name: String,
	value: String,
	raws: Raws,
}

#[derive(Debug)]
struct Comment {
	indicator: String,
	value: String,
	raws: Raws,
}
