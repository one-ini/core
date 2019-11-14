//! # EditorConfig INI
//!
//! `editorconfig_ini` is a collection of utilities that handle the parsing of
//! EditorConfig-INI file contents into AST, modifying the AST and serializing
//! the result back into a string.

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "ini.pest"]
pub struct INIParser;

/// Parses EditorConfig-INI contents into AST.
///
/// # Examples
///
/// ```
/// let contents = "root=true";
/// let ast = editorconfig_ini::parse(contents);
/// ```
pub fn parse(contents: &String) -> std::collections::HashMap<
	&str,
	std::collections::HashMap<&str, &str>
> {
	let file = INIParser::parse(Rule::file, contents)
		.expect("unsuccessful parse") // unwrap the parse result
		.next().unwrap(); // get and unwrap the `file` rule; never fails

	let mut properties: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

	let mut current_section_name = "";

	for line in file.into_inner() {
		match line.as_rule() {
			Rule::section => {
				let mut inner_rules = line.into_inner(); // { name }
				current_section_name = inner_rules.next().unwrap().as_str();
			}
			Rule::property => {
				let mut inner_rules = line.into_inner(); // { name ~ "=" ~ value }

				let name: &str = inner_rules.next().unwrap().as_str();
				let value: &str = inner_rules.next().unwrap().as_str();

				// Insert an empty inner hash map if the outer hash map hasn't
				// seen this section name before.
				let section = properties.entry(current_section_name).or_default();
				section.insert(name, value);
			}
			Rule::EOI => (),
			_ => unreachable!(),
		}
	}

	return properties;
}
