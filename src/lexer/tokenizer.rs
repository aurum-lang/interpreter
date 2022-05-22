#![allow(dead_code)]
use super::types::{ Keywords, match_keyword };

pub fn tokenize<T: ToString>(t: T) -> Vec<u8> {
	let file: String = parse(t.to_string());
	let mut buffer: String = String::new();

	for line in file.lines() {
		
	}

	buffer.into_bytes()
}

fn parse(s: String) -> String {
	let mut buffer: String = String::new();

	for line in s.lines() {
		if line.trim() == "" || line.starts_with("//") { continue; }

		if line.ends_with(';') {
			buffer.push_str(line.strip_suffix(';').unwrap());
			buffer.push('\n');
		} else {
			buffer.push_str(line);
			buffer.push('\n');
		}
	}
	buffer
}

fn split(line: &str) -> (String, String, String) {
	let mut split1 = line.split(' ').collect::<Vec<&str>>();
	let vartype = split1.remove(0);
	let body = split1.concat();

	let mut split2: Vec<&str> = body.split('=').collect::<Vec<&str>>();
	let name = split2.remove(0);
	let value = split2.concat();
	
	(vartype.into(), name.into(), value)
}