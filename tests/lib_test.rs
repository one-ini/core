#[cfg(test)]
mod tests {
	use one_ini::parse;

	use std::fs;

	#[test]
	fn it_works() {
		let contents = fs::read_to_string("tests/fixtures/config.ini").unwrap();
		let ast = parse(&contents).unwrap();
		assert_eq!(ast.to_string(), contents);
	}
}
