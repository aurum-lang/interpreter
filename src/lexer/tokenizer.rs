#![allow(dead_code)]
use super::types::{ Keywords, match_keyword };

pub fn tokenize<T: ToString>(t: T) -> Vec<u8> {
	let file: String = parse(t.to_string());
	let mut buffer: String = String::new();

	for line in file.lines() {}

	buffer.into_bytes()
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

fn split(line: &str) -> (&str, &str, &str) {
	let mut split: Vec<&str> = line.split('=').collect::<Vec<&str>>();
	let name = split.remove(0);
	
	todo!()
}