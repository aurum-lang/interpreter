mod lexer;

#[cfg(test)]
mod tests {
    #[test]
    fn lexer() {
        let read: String = std::fs::read_to_string("test.au").unwrap();
		let result: Vec<u8> = crate::lexer::tokenize(read).unwrap();

		let _ = std::fs::create_dir("compile/");
		let _ = std::fs::write("compile/c.lex", result);
    }
}
