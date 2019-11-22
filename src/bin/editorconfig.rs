extern crate clap;
extern crate editorconfig;

use clap::{App, Arg};
use std::fs;

fn main() {
	let matches = App::new("EditorConfig")
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
	let unparsed_file = fs::read_to_string(ini_path).expect("cannot read file");

	println!(
		"{:#?}",
		editorconfig::parse(&unparsed_file).unwrap().to_string()
	);
}
