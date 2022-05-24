#![allow(dead_code)]
use super::types::{ GenericError, match_symbol };
use regex::Regex;

pub fn tokenize<T: ToString>(t: T) -> Result<Vec<u8>, GenericError> {
	let file: String = parse(t.to_string());
	let mut buffer: String = String::new();
	let mut ln: i32 = 1;

	for line in file.lines() {
		if line.trim() == "" || line.starts_with("//") {
			ln += 1;
			continue;
		}  // Moved to preserve line count
		let line = line.trim();

		let words: Vec<&str> = line.split(' ').collect::<Vec<&str>>();
		if words[0].starts_with("print") {
			let word = words[0];
			let expr = Regex::new(r"\(.*?\)").unwrap();
			let res = expr.find(word);
				
			if let Some(m) = res {
				buffer.push_str("<Keyword(print)>");
				buffer.push_str(format!(
					"<{}>",
					get_type(&word[m.range()].replace('(', "").replace(')', "")),
				).as_str());
				buffer.push('\n');
			}
			continue;
		}
		match words[0] {
			"int" | "bool" | "str" => {
				let assignment = words[1..].join(" ");
				let sides: Vec<&str> = assignment.split('=').collect();
				if sides.len() != 2 {
					return Err(GenericError { msg: format!("Invalid variable declaration at line {ln}.") })
				}
				let (name, body) = (sides[0].trim(), sides[1].trim());

				buffer.push_str(format!("<Keyword({})><Identifier({})><{}>", words[0], name, get_type(body)).as_str());
				buffer.push('\n');
			},
			"int[]" => {
				let assignment = words[1..].concat();
				let sides: Vec<&str> = assignment.split('=').collect();
				if sides.len() != 2 {
					return Err(GenericError { msg: format!("Invalid variable declaration at line {ln}.") })
				}
				let (name, body) = (sides[0].trim(), sides[1].trim());

				buffer.push_str(format!("<Keyword(intarr)><Identifier({})><{}>", name, body).as_str());
				buffer.push('\n');
			},
			"bool[]" => {
				let assignment = words[1..].concat();
				let sides: Vec<&str> = assignment.split('=').collect();
				if sides.len() != 2 {
					return Err(GenericError { msg: format!("Invalid variable declaration at line {ln}.") })
				}
				let (name, body) = (sides[0].trim(), sides[1].trim());

				buffer.push_str(format!("<Keyword(boolarr)><Identifier({})><{}>", name, body).as_str());
				buffer.push('\n');
			},
			"str[]" => {
				let assignment = words[1..].concat();
				let sides: Vec<&str> = assignment.split('=').collect();
				if sides.len() != 2 {
					return Err(GenericError { msg: format!("Invalid variable declaration at line {ln}.") })
				}
				let (name, body) = (sides[0].trim(), sides[1].trim());

				buffer.push_str(format!("<Keyword(strarr)><Identifier({})><{}>", name, body).as_str());
				buffer.push('\n');
			},
			"for" => {
				buffer.push_str("<Keyword(for)>");
				buffer.push_str(format!("<Identifier({})>", words[1]).as_str());
				buffer.push_str(format!("<Keyword({})>", words[2]).as_str());
				if words[3].parse::<i32>().is_ok() {
					if words.len() != 7 {
						return Err(GenericError { msg: format!("Malformed for loop at line {ln}.") })
					}
					for w in (words[3..]).iter() {
						if let Some(res) = match_symbol(w) {
							buffer.push_str(format!("<{}>", res).as_str());
						} else {
							buffer.push_str(format!("<{}>", get_type(w)).as_str());
						}
					}
					// buffer.push_str(format!("<Literal({})>", words[3]).as_str());
				} else {
					if words.len() != 5 {
						return Err(GenericError { msg: format!("Malformed for loop at line {ln}.") })
					}
					for w in (words[3..]).iter() {
						if let Some(res) = match_symbol(w) {
							buffer.push_str(format!("<{}>", res).as_str());
						} else {
							buffer.push_str(format!("<{}>", get_type(w)).as_str());
						}
					}
				}
				buffer.push('\n');
			},
			"fn" => {
				buffer.push_str("<Keyword(function)>");
				// if !words[1].ends_with("()") {
				// 	return Err(GenericError { msg: format!("Incomplete function definition at line {ln}.") })
				// }
				buffer.push_str(format!("<Identifier({})>", words[1].replace('(', "").replace(')', "")).as_str());
				// buffer.push_str("<LParen>");
				// buffer.push_str("<RParen>");
				
				for w in words[2..].iter() {
					if let Some(res) = match_symbol(w) {
						buffer.push_str(format!("<{}>", res).as_str());
					} else {
						buffer.push_str(format!("<{}>", get_type(w)).as_str());
					}
				}
				buffer.push('\n');
			},
			"if" => {
				buffer.push_str("<Keyword(if)>");
				for w in (words[1..]).iter() {
					if let Some(res) = match_symbol(w) {
						buffer.push_str(format!("<{}>", res).as_str());
					} else {
						buffer.push_str(format!("<{}>", get_type(w)).as_str());
					}
				}
				buffer.push('\n');
			},
			"print" => {
				let word = words[1];
				let expr = Regex::new(r"/\(.*?\)/g").unwrap();
				let res = expr.find(word);
				
				if let Some(m) = res {
					buffer.push_str("<Keyword(print)>");
					buffer.push_str(get_type(&word[m.range()]).as_str());
				}
				buffer.push('\n');
			},
			_ => {
				if let Some(res) = match_symbol(line) {
					buffer.push_str(format!("<{}>", res).as_str());
					buffer.push('\n');
				}
			}
		}
		ln += 1;
	}

	Ok(buffer.into_bytes())
}

fn parse(s: String) -> String {
	let mut buffer: String = String::new();

	for line in s.lines() {
		// if line.trim() == "" || line.starts_with("//") { continue; }  // Moved

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

fn get_type(word: &str) -> String {
	let trim = word.trim();
	if word == "true" || word == "false" {
		format!("BoolLiteral({word})")
	} else if word.parse::<i32>().is_ok() {
		format!("IntLiteral({word})")
	} else if trim.starts_with('"') && trim.ends_with('"') {
		format!("StringLiteral({})", word.replace('"', ""))
	} else {
		format!("Keyword({word})")
	}
	// if word == "true" || word == "false" || word.parse::<i32>().is_ok() || 
	// (trim.starts_with('"') && trim.ends_with('"')) {
	// 	format!("Literal({word})")
	// } else {
	// 	format!("Keyword({word})")
	// }
}