#![allow(dead_code)]
use super::types::{ Keywords, match_keyword, GenericError };

pub fn tokenize<T: ToString>(t: T) -> Result<Vec<u8>, GenericError> {
	let file: String = parse(t.to_string());
	let mut buffer: String = String::new();

	for line in file.lines() {
		let words: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
			match words[0] {
			"int" | "bool" | "str" => {
				let assignment = words[1..].concat();
				let sides: Vec<&str> = assignment.split('=').collect();
				if sides.len() != 2 {
					return Err(GenericError {})
				}
				let (name, body) = (sides[0].trim(), sides[1].trim());

				buffer.push_str(format!("<{}><Ident({})><{}>", words[0], name, body).as_str());
				buffer.push('\n');
			},
			"int[]" => {
				let assignment = words[1..].concat();
				let sides: Vec<&str> = assignment.split('=').collect();
				if sides.len() != 2 {
					return Err(GenericError {})
				}
				let (name, body) = (sides[0].trim(), sides[1].trim());

				buffer.push_str(format!("<intarr><Ident({})><{}>", name, body).as_str());
				buffer.push('\n');
			},
			"bool[]" => {
				let assignment = words[1..].concat();
				let sides: Vec<&str> = assignment.split('=').collect();
				if sides.len() != 2 {
					return Err(GenericError {})
				}
				let (name, body) = (sides[0].trim(), sides[1].trim());

				buffer.push_str(format!("<boolarr><Ident({})><{}>", name, body).as_str());
				buffer.push('\n');
			},
			"str[]" => {
				let assignment = words[1..].concat();
				let sides: Vec<&str> = assignment.split('=').collect();
				if sides.len() != 2 {
					return Err(GenericError {})
				}
				let (name, body) = (sides[0].trim(), sides[1].trim());

				buffer.push_str(format!("<strarr><Ident({})><{}>", name, body).as_str());
				buffer.push('\n');
			},
			_ => ()
		}
	}

	Ok(buffer.into_bytes())
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