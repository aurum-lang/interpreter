#![allow(dead_code)]
use super::types::{ Keywords, match_keyword };

pub fn tokenize<T: ToString>(t: T) -> Vec<u8> {
	let file: String = parse(t.to_string());
	todo!()
}

fn parse(s: String) -> String {
	let mut buffer: String = String::new();

	for line in s.lines() {
		if line.trim() == "" || line.starts_with("//") { continue; }
		
		if line.ends_with(';') {
			buffer.push_str(line.strip_suffix(';').unwrap());
		} else {
			buffer.push_str(line);
		}
	}
	buffer
}