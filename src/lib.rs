mod lexer;

#[cfg(test)]
mod tests {
	use crate::lexer;

    #[test]
    fn lexer_test() {
        let read: String = std::fs::read_to_string("test.au").unwrap();
		let result: Vec<u8> = lexer::tokenize(read).unwrap_or_else(|e| {
			println!("{}", e.msg);
			std::process::exit(1);
		});

		let _ = std::fs::create_dir("compile/");
		let _ = std::fs::write("compile/c.lex", result);
    }
}
