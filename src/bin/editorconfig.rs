extern crate clap;
extern crate editorconfig_ini;

use clap::{App, Arg};
use std::fs;

fn main() {
	let matches = App::new("EditorConfig")
		.version(env!("CARGO_PKG_VERSION"))
		.author("Jed Mao <jedmao@outlook.com>")
		.about("Gets a configuration for a file path")
		.arg(
			Arg::with_name("file_path")
				.help("The file for which you want a configuration")
				.required(true),
		)
		.get_matches();

	let file_path = matches.value_of("file_path").unwrap();
	let unparsed_file = fs::read(file_path).expect("cannot read file");

	println!(
		"{}",
		editorconfig_ini::parse(&unparsed_file).unwrap().to_string()
	);
}
