use std::fmt;
use std::iter::Peekable;
use std::str::Chars;

use crate::interpret_bool::*;
use crate::interpret_num::*;

#[derive(PartialEq, Debug)]
pub enum TokenKind {
    Number,
    Boolean,

    // Keywords
    Cond,
    Define,
    List,
    Cons,
    Empty,
    Car,
    Cdr,
    EmptyHuh,
    ListHuh,

    Identifier,

    // Special characters
    OpenParen,
    CloseParen,

    // Operators
    Plus,
    Minus,
    Slash,
    Star,
    Percent,

    Ampersand,
    Pipe,
    Bang,

    LessThan,
    Equal,
    GreaterThan,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}({})", self.kind, self.text)
    }
}

#[derive(PartialEq, Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub text: String,
}

#[derive(Debug)]
pub struct TokenIter<'a> {
    tokens: &'a Vec<Token>,
    index: usize,
}

impl<'a> TokenIter<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        TokenIter { tokens, index: 0 }
    }
}

impl<'a> Iterator for TokenIter<'a> {
    type Item = &'a Token;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.tokens.len() {
            let token = &self.tokens[self.index];
            self.index += 1;
            Some(token)
        } else {
            None
        }
    }
}

pub fn string_to_tokens(s: String) -> Vec<Token> {
    let mut result = Vec::new();
    let mut s_iterator = s.chars().into_iter().peekable();

    consume_whitespace(&mut s_iterator);
    while s_iterator.peek().is_some() {
        let token = token_from_position(&mut s_iterator);
        result.push(token);
        consume_whitespace(&mut s_iterator);
    }

    result
}

pub fn token_kind_to_binary_num_op(kind: &TokenKind) -> BinaryNumOp {
    match kind {
        TokenKind::Plus => BinaryNumOp::Add,
        TokenKind::Minus => BinaryNumOp::Sub,
        TokenKind::Slash => BinaryNumOp::Div,
        TokenKind::Star => BinaryNumOp::Mul,
        TokenKind::Percent => BinaryNumOp::Mod,
        _ => panic!("Could not parse token kind to binary num op"),
    }
}

pub fn token_kind_to_binary_bool_op(kind: &TokenKind) -> BinaryBoolOp {
    match kind {
        TokenKind::Ampersand => BinaryBoolOp::And,
        TokenKind::Pipe => BinaryBoolOp::Or,
        _ => {
            panic!("Could not parse token kind to binary bool op");
        }
    }
}

pub fn token_kind_to_unary_bool_op(kind: &TokenKind) -> UnaryBoolOp {
    match kind {
        TokenKind::Bang => UnaryBoolOp::Not,
        _ => panic!("Could not parse token kind to unary bool op"),
    }
}

pub fn token_kind_to_cmp_bool_op(kind: &TokenKind) -> CmpBoolOp {
    match kind {
        TokenKind::LessThan => CmpBoolOp::Lt,
        TokenKind::Equal => CmpBoolOp::Eq,
        TokenKind::GreaterThan => CmpBoolOp::Gt,
        _ => panic!("Could not parse token kind to cmp bool op"),
    }
}

fn is_valid_starting_string_token_char(c: &char) -> bool {
    c.is_ascii_alphabetic() || *c == '_'
}

fn is_valid_string_token_char(c: &char) -> bool {
    is_valid_starting_string_token_char(c) || c.is_ascii_alphanumeric() || *c == '?' || *c == '!' || *c == '-' || *c == ':'
}

fn token_from_position(s: &mut std::iter::Peekable<std::str::Chars>) -> Token {
    if s.peek().is_none() {
        panic!("Index out of range while parsing tokens.")
    }

    if s.peek().unwrap().is_numeric() {
        let mut number_buff = String::new();
        while s.peek().is_some() && s.peek().unwrap().is_numeric() {
            number_buff.push(s.next().unwrap());
        }

        return Token {
            kind: TokenKind::Number,
            text: number_buff,
        };
    }

    if is_valid_starting_string_token_char(s.peek().unwrap()) {
        let mut buff = String::new();
        while s.peek().is_some() && (is_valid_string_token_char(s.peek().unwrap())) {
            buff.push(s.next().unwrap());
        }

        return match buff.as_str() {
            "true" | "false" => Token {
                kind: TokenKind::Boolean,
                text: buff,
            },
            "cond" => Token {
                kind: TokenKind::Cond,
                text: buff,
            },
            "define" => Token {
                kind: TokenKind::Define,
                text: buff,
            },
            "list" => Token {
                kind: TokenKind::List,
                text: buff,
            },
            "cons" => Token {
                kind: TokenKind::Cons,
                text: buff,
            },
            "empty" => Token {
                kind: TokenKind::Empty,
                text: buff,
            },
            "car" => Token {
                kind: TokenKind::Car,
                text: buff,
            },
            "cdr" => Token {
                kind: TokenKind::Cdr,
                text: buff,
            },
            "empty?" => Token {
                kind: TokenKind::EmptyHuh,
                text: buff,
            },
            "list?" => Token {
                kind: TokenKind::ListHuh,
                text: buff,
            },
            _ => Token {
                kind: TokenKind::Identifier,
                text: buff,
            },
        };
    }

    match s.next().unwrap() {
        '(' | '[' => Token {
            kind: TokenKind::OpenParen,
            text: "(".to_string(),
        },
        ')' | ']' => Token {
            kind: TokenKind::CloseParen,
            text: ")".to_string(),
        },
        '+' => Token {
            kind: TokenKind::Plus,
            text: "+".to_string(),
        },
        '-' => Token {
            kind: TokenKind::Minus,
            text: "-".to_string(),
        },
        '/' => Token {
            kind: TokenKind::Slash,
            text: "/".to_string(),
        },
        '*' => Token {
            kind: TokenKind::Star,
            text: "*".to_string(),
        },
        '%' => Token {
            kind: TokenKind::Percent,
            text: "%".to_string(),
        },
        '&' => Token {
            kind: TokenKind::Ampersand,
            text: "&".to_string(),
        },
        '|' => Token {
            kind: TokenKind::Pipe,
            text: "|".to_string(),
        },
        '!' => Token {
            kind: TokenKind::Bang,
            text: "!".to_string(),
        },
        '<' => Token {
            kind: TokenKind::LessThan,
            text: "<".to_string(),
        },
        '=' => Token {
            kind: TokenKind::Equal,
            text: "=".to_string(),
        },
        '>' => Token {
            kind: TokenKind::GreaterThan,
            text: ">".to_string(),
        },
        x => todo!("Unhandled case in token_from_position {}", x),
    }
}

fn consume_whitespace(it: &mut Peekable<Chars<'_>>) {
    while it.peek().is_some_and(|x| x.is_whitespace()) {
        it.next();
    }
}
