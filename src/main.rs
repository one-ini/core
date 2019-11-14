extern crate clap;
use clap::{App, Arg};
use std::fs;
use std::collections::HashMap;

extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "ini.pest"]
pub struct INIParser;

fn main() {
	let matches = App::new("EditorConfig-INI Parser")
		.version("0.1.0")
		.author("Jed Mao <jedmao@outlook.com>")
		.about("Parses an INI file into AST")
		.arg(
			Arg::with_name("ini_path")
				.help("Sets the INI file path to read")
				.required(true),
		)
		.get_matches();

	let ini_path = matches.value_of("ini_path").unwrap();
	println!("ini_path: {}", ini_path);

	let unparsed_file = fs::read_to_string(ini_path).expect("cannot read file");

	let file = INIParser::parse(Rule::file, &unparsed_file)
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

	println!("{:#?}", properties);
}
