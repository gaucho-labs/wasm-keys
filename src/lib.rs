pub mod wasm_key;
pub mod hotkey;

pub mod key_map;


pub trait Fluid<T> {
    fn from_str(key_combination: &str) -> T;
}

/*
pub use std::fmt;

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum ParseError {
    UnidentifiedKey(String),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParseError::UnidentifiedKey(s) => write!(f, "Unidentified key: {}", s),
        }
    }
}

impl std::error::Error for ParseError {}
 */