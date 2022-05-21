#![allow(unused_variables, dead_code, clippy::single_match)]
use std::num::ParseIntError;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct GenericError {}

impl std::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_str("Generic error thrown.");
        Ok(())
    }
}

impl Error for GenericError {}

impl From<ParseIntError> for GenericError {
    fn from(_: ParseIntError) -> Self {
        Self {}
    }
}

////////////

pub enum Keywords {
	For,
	Func,
	Integer,
	IntArray,
	StringArray,
	BoolArray,
	Bool,
	Concurrent,
	String,
	If,
	Ident(String)
}

fn match_keyword(kw: &str) -> Keywords {
	match kw {
		"for" => Keywords::For,
		"fn" => Keywords::Func,
		"int" => Keywords::Integer,
		"int[]" => Keywords::IntArray,
		"str[]" => Keywords::StringArray,
		"bool[]" => Keywords::BoolArray,
		"bool" => Keywords::Bool,
		"conc" => Keywords::Concurrent,
		"str" => Keywords::String,
		"if" => Keywords::If,
		_ => Keywords::Ident(kw.to_owned())
	}
}

////////////////

pub fn tokenize<T: ToString>(t: T) -> Result<Vec<u8>, GenericError> {
	let file: String = t.to_string();
	let mut output: String = String::new();

	for line in file.lines() {
		if line.trim() == "" || line.starts_with("//") {
			continue;
		}

		let mut line = line;
		if line.ends_with(';') {
			line = &line[..line.len() - 1]
		}
		let mut split = line.split(' ').collect::<Vec<&str>>();
		let kw = split.remove(0);
		let body = split.join("");
		match match_keyword(kw) {
			Keywords::Integer => {
				let (name, val) = parse_int(&body)?;
				output.push_str(format!("<int;name={name};val={val}>\n").as_str());
			},
			Keywords::Bool => {
				let (name, val) = parse_bool(&body)?;
				output.push_str(format!("<bool;name={name};val={val}>\n").as_str());
			},
			Keywords::String => {
				let (name, val) = parse_string(&body)?;
				output.push_str(format!("<string;name={name};val={val}>\n").as_str());
			},
			Keywords::IntArray => {
				let (name, val) = parse_string(&body)?;
				output.push_str(format!("<intarr;name={name};val={val}>\n").as_str());
			},
			Keywords::StringArray => {
				todo!("String arrays are not implemented yet!");
				// let (name, val) = parse_string(&body)?;
				// output.push_str(format!("<strarr;name={name};val={val}>\n").as_str());
			},
			Keywords::BoolArray => {
				let (name, val) = parse_string(&body)?;
				output.push_str(format!("<boolarr;name={name};val={val}>\n").as_str());
			},
			_ => ()
		}
	}

	Ok(output.into_bytes())
}

////////////////

fn parse_int(line: &str) -> Result<(String, i32), ParseIntError> {
	let t = line.replace(' ', "");  // Strip whitespace 
	let split = t.split('=').collect::<Vec<&str>>();

	let (name, raw_type) = (split[0], split[1]);
	let rich_type = raw_type.parse::<i32>()?;

	Ok( (name.to_owned(), rich_type) )
}

fn parse_bool(line: &str) -> Result<(String, bool), GenericError> {
	let t = line.replace(' ', "");  // Strip whitespace 
	let split = t.split('=').collect::<Vec<&str>>();

	let (name, raw_type) = (split[0], split[1]);
	let rich_type = {
		if raw_type == "true" {
			Ok(true)
		} else if raw_type == "false" {
			Ok(false)
		} else {
			Err(GenericError {})
		}
	}?;

	Ok( (name.to_owned(), rich_type) )
}

fn parse_string(line: &str) -> Result<(String, String), GenericError> {
	let t = line.replace(' ', "");  // Strip whitespace 
	let split = t.split('=').collect::<Vec<&str>>();

	let (name, raw_type) = (split[0], split[1]);
	let rich_type = raw_type.to_string();

	Ok( (name.to_owned(), rich_type) )
}

// We'll actually parse the array in the AST step
// fn parse_arr(line: &str) -> Result<(String, String), GenericError> {
// 	let t = line.replace(' ', "");  // Strip whitespace 
// 	let split = t.split('=').collect::<Vec<&str>>();

// 	let (name, raw_type) = (split[0], split[1]);
// 	let rich_type = raw_type.to_string();

// 	Ok( (name.to_owned(), rich_type) )
// }