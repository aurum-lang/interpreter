#![allow(unused_variables, dead_code)]
use std::num::ParseIntError;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct GenericError {
	pub msg: String
}

impl std::fmt::Display for GenericError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		let _ = f.write_str("Generic error thrown.");
        Ok(())
    }
}

impl Error for GenericError {}

impl From<ParseIntError> for GenericError {
    fn from(_: ParseIntError) -> Self {
        Self { msg: "".into() }
    }
}

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
	Ident(String),
	Print
}

pub fn match_keyword(kw: &str) -> Keywords {
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
		"print" => Keywords::Print,
		_ => Keywords::Ident(kw.to_owned())
	}
}

pub fn match_symbol(c: &str) -> Option<String> {
	match c {
		"{" => Some("LBracket".into()),
		"}" => Some("RBracket".into()),
		"(" => Some("LParen".into()),
		")" => Some("RParen".into()),
		"==" => Some("EqualTo".into()),
		"=" => Some("Assign".into()),
		"+=" => Some("AssignAdd".into()),
		"-=" => Some("AssignSubtract".into()),
		"/=" => Some("AssignDivide".into()),
		"*=" => Some("AssignMultiply".into()),
		_ => None
	}
}