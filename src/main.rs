extern crate clap;
use clap::{App, Arg};
use std::fs;

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

	println!("With text:\n{}", fs::read_to_string(ini_path).unwrap());
}
