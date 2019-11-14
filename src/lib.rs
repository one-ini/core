//! # EditorConfig
//!
//! `editorconfig` is a collection of utilities that handle the parsing of
//! EditorConfig-INI file contents into AST, modifying the AST and serializing
//! the result back into a string.

extern crate pest;
#[macro_use]
extern crate pest_derive;

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
pub fn parse(contents: &String) -> EditorConfigINIAST {
	let file = INIParser::parse(Rule::file, contents)
		.expect("unsuccessful parse")
		.next()
		.unwrap();
	let mut section = Section {
		kind: "section",
		name: "prelude",
		props: Vec::new(),
	};
	let sections: Vec<Section> = Vec::new();
	sections.append(section);
	let ast = EditorConfigINIAST {
		kind: "EditorConfigINI",
		version: "0.1.2",
		props: Vec::new(),
		sections,
	};

	for line in file.into_inner() {
		match line.as_rule() {
			Rule::section => {
				section = Section {
					kind: "section",
					name: line.into_inner().next().unwrap().as_str(),
					props: Vec::new(),
				};
				ast.sections.append(&section);
			}
			Rule::property => {
				let inner_rules = line.into_inner();
				let prop = Property {
					kind: "property",
					name: inner_rules.next().unwrap().as_str(),
					value: inner_rules.next().unwrap().as_str(),
				};
				section.props.append(prop);
			}
			Rule::EOI => (),
			_ => unreachable!(),
		}
	}

	return ast;
}

#[derive(Debug)]
struct EditorConfigINIAST<'a, 'b> {
	kind: &'a str,
	version: &'a str,
	props: Vec<&'b Token<'a>>,
	sections: Vec<Section<'a, 'b>>,
}

#[derive(Debug)]
struct Section<'a, 'b> {
	kind: &'a str,
	name: &'a str,
	props: Vec<&'b Token<'a>>,
}

#[derive(Debug)]
struct Property<'a> {
	kind: &'a str,
	name: &'a str,
	value: &'a str,
}

#[derive(Debug)]
struct Token<'a> {
	kind: &'a str, // pretty(&self) -> String;
	               // toAST(&self) -> String;
	               // toString(&self) -> String;
}
