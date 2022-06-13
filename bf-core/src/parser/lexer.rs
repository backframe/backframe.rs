#![allow(unused)]
use regex::Regex;

// static SPEC: Vec<(&'static str, &'static str)> = vec![(r"^\w+", "None"), (r"^\#.*", "None")];

pub struct Lexer {
    pub string: &'static str,
    pub cursor: i32,
}

pub struct Token {
    _type: &'static str,
    value: &'static str,
}

impl Lexer {
    pub fn new(val: &'static str) -> Self {
        Self {
            string: val,
            cursor: 0,
        }
    }

    pub fn has_more_tokens(&self) -> bool {
        self.cursor < self.string.len() as i32
    }

    pub fn is_eof(&self) -> bool {
        self.cursor == self.string.len() as i32
    }

    fn _match(&mut self, re: Regex, val: &'static str) -> Option<&str> {
        let matches = re.captures(val);
        if matches.is_none() {
            return None;
        }
        let matches = matches.unwrap();

        let value = matches.get(0).unwrap().as_str();
        self.cursor += value.len() as i32;

        Some(value)
    }
}
