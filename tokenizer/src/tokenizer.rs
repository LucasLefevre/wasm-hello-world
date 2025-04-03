use std::{sync::Arc, thread::{self, JoinHandle}};

use once_cell::sync::Lazy;
use regex::Regex;

use serde::Serialize;


#[derive(Debug, Clone, Copy, Serialize)]
pub enum TokenType {
    Operator=1,
    Number=2,
    String=3,
    Symbol=4,
    Space=5,
    Debugger=6,
    ArgSeparator=7,
    LeftParen=8,
    RightParen=9,
    Reference=10,
    InvalidReference=11,
    Unknown=12,
}

#[derive(Debug, Serialize)]
pub struct Token {
    pub token_type: TokenType,
    pub value: String,
}

pub struct TokenizingChars<'a> {
    current: Option<char>,
    input: &'a str,
    index: usize,
}

impl<'a> TokenizingChars<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input, index: 0, current: input.chars().next() }
    }

    pub fn shift(&mut self) -> Option<char> {
        let ch = self.current?;
        self.input = &self.input[ch.len_utf8()..];
        self.current = self.input.chars().next();
        Some(ch)
    }

    pub fn is_over(&self) -> bool {
        self.input.len() <= 0
    }

    pub fn current_starts_with(&self, s: &str) -> bool {
        self.input.starts_with(s)
    }

    pub fn advance_by(&mut self, n: usize) {
        // careful about unicode special characters!
        self.index += n;
        self.input = &self.input[n..];
        self.current = self.input.chars().next();
    }

    pub fn remaining(&self) -> &'a str {
        &self.input
    }
}

const OPERATORS: [&str; 12] = [
    "+", "-", "*", "/", ":", "=", "<>", ">=", ">", "<=", "<", "^",
];

pub fn parallel_tokenize(inputs: Vec<String>) -> Vec<Vec<Token>> {
    let formulas: Arc<Vec<String>> = Arc::from(inputs);
    let num_threads = 10;
    let chunk_size = formulas.len() / num_threads;

    let mut handles: Vec<JoinHandle<_>> = vec![];
    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = (start + chunk_size).min(formulas.len());
        if start >= formulas.len() {
            break;
        }
        let chunk = Arc::clone(&formulas); // No actual copying
        let handle = thread::spawn(move || chunk[start..end].iter().map(|formula| tokenize(formula)).collect::<Vec<_>>());
        handles.push(handle);
    }

    let mut tokens = Vec::new();
    for handle in handles {
        tokens.extend(handle.join().unwrap());
    }
    tokens
}


pub fn tokenize(formula: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = TokenizingChars::new(formula);
    while !chars.is_over() {
        if let Some(token) = tokenize_space(&mut chars)
            .or_else(|| tokenize_operator(&mut chars))
            .or_else(|| tokenize_arg_separator(&mut chars))
            .or_else(|| tokenize_parenthesis(&mut chars))
            .or_else(|| tokenize_string(&mut chars))
            .or_else(|| tokenize_debugger(&mut chars))
            .or_else(|| tokenize_invalid_range(&mut chars))
            .or_else(|| tokenize_number(&mut chars))
            .or_else(|| tokenize_symbol(&mut chars))
        {
            tokens.push(token);
        } else {
            tokens.push(Token {
                token_type: TokenType::Unknown,
                value: chars.shift().map(|c| c.to_string()).unwrap_or(" ".to_string())
            })
        }
    }
    tokens
}

pub fn encode_tokens_to_bytes(tokens_vec: Vec<Vec<Token>>) -> Vec<u8> {
    let number_of_tokens = tokens_vec.iter().map(|tokens| tokens.len()).sum::<usize>();
    let mut buffer = vec![0; number_of_tokens*2 + tokens_vec.len() +1];

    let mut processed_inputs = 0;
    for tokens in tokens_vec.iter() {
        for (i, token) in tokens.iter().enumerate() {
            let start = processed_inputs + 2*i;
            buffer[start] = token.token_type as u8;
            buffer[start+1] = token.value.len() as u8;
        }
        processed_inputs += tokens.len()*2;
        buffer[processed_inputs] = 0;
        processed_inputs += 1;
    }
    buffer
}

fn tokenize_space(chars: &mut TokenizingChars) -> Option<Token> {
    let mut i = 0;
    while chars.current?.is_whitespace() {
        i += 1;
        chars.shift();
    }
    if i > 0 {
        return Some(Token {
            token_type: TokenType::Space,
            value: " ".repeat(i),
        });
    }
    None
}

fn tokenize_operator(chars: &mut TokenizingChars) -> Option<Token> {
    for op in OPERATORS.into_iter() {
        let len = op.len();
        let remaining = chars.remaining();
        if remaining.len() >= len && op == &remaining[..len] {
            chars.advance_by(len);
            return Some(Token {
                token_type: TokenType::Operator,
                value: op.to_string(),
            });
        }
    }
    None
}

fn tokenize_arg_separator(chars: &mut TokenizingChars) -> Option<Token> {
    match chars.current {
        Some(',') => {
            chars.shift();
            Some(Token {
                token_type: TokenType::ArgSeparator,
                value: ",".to_string(),
            })
        }
        _ => None,
    }
}

fn tokenize_parenthesis(chars: &mut TokenizingChars) -> Option<Token> {
    match chars.current {
        Some('(') => {
            chars.shift();
            Some(Token {
                token_type: TokenType::LeftParen,
                value: "(".to_string(),
            })
        }
        Some(')') => {
            chars.shift();
            Some(Token {
                token_type: TokenType::RightParen,
                value: ")".to_string(),
            })
        }
        _ => None,
    }
}

fn tokenize_string(chars: &mut TokenizingChars) -> Option<Token> {
    if chars.current == Some('"') {
        let mut l = 0;
        while chars.remaining().chars().nth(l + 1) != None
            && chars.remaining().chars().nth(l + 1) != Some('"')
            || chars.remaining().chars().nth(l) == Some('\\')
        {
            l += 1;
        }
        if l > 0 && chars.remaining().chars().nth(l + 1) == Some('"') {
            l += 2; // 2 quotes
            let value = chars.remaining()[..l].to_string();
            chars.advance_by(l);
            return Some(Token {
                token_type: TokenType::String,
                value,
            });
        }
    }
    None
}

fn tokenize_debugger(chars: &mut TokenizingChars) -> Option<Token> {
    match chars.current {
        Some('?') => {
            chars.shift();
            Some(Token {
                token_type: TokenType::Debugger,
                value: "?".to_string(),
            })
        }
        _ => None,
    }
}

fn tokenize_invalid_range(chars: &mut TokenizingChars) -> Option<Token> {
    if chars.current_starts_with("#REF") {
        return Some(Token {
            token_type: TokenType::InvalidReference,
            value: "#REF".to_string(),
        });
    }
    None
}

static RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?:^-?\d+(?:\.?\d*(?:e\d+)?)?|^-?\.\d+)").unwrap());
// static RE: Lazy<Regex> = Lazy::new(||Regex::new(r"(?:^-?\d+(?:\.?\d*(?:e\d+)?)?|^-?\.\d+)(?!\w|!)").unwrap());

fn tokenize_number(chars: &mut TokenizingChars) -> Option<Token> {
    match chars.current {
        // first check if the first character is valid
        Some('0'..='9' | '.') => {
            let Some(number_match) = RE.find(chars.remaining()) else {
                return None;
            };
            let value = number_match.as_str().to_string();
            chars.advance_by(value.len());
            Some(Token {
                token_type: TokenType::Number,
                value,
            })
        }
        _ => None,
    }
}

fn tokenize_symbol(chars: &mut TokenizingChars) -> Option<Token> {
    match chars.current {
        Some('\'') => {
            return tokenize_quoted_symbol(chars)
        }
        _ => {
            None
        },
    }
}

fn tokenize_quoted_symbol(chars: &mut TokenizingChars) -> Option<Token> {
    None
}
