use std::fmt::Formatter;
use std::num::ParseFloatError;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Null,
    True,
    False,
    Number(f64),
    String(String),
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    EndOfFile,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Token::Null => write!(f, "null"),
            Token::True => write!(f, "true"),
            Token::False => write!(f, "false"),
            Token::Number(n) => write!(f, "Number({})", n),
            Token::String(s) => write!(f, "String({})", s),
            Token::LeftBrace => write!(f, "{{"),
            Token::RightBrace => write!(f, "}}"),
            Token::LeftBracket => write!(f, "["),
            Token::RightBracket => write!(f, "]"),
            Token::Comma => write!(f, ","),
            Token::Colon => write!(f, ":"),
            Token::EndOfFile => write!(f, "EOF"),
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum TokenizerError {
    #[error("Invalid character `{0}`")]
    InvalidCharacter(char),
    #[error("Unexpected char `{0}`")]
    UnexpectedToken(char),
    #[error("Expected a valid number `{0}`")]
    ParseNumberError(#[from] ParseFloatError),
    #[error("Expected null token")]
    ExpectedNull,
    #[error("Expected a bool `{0}` but got `{1}`")]
    ParseBooleanError(bool, String),
    #[error("Expected char `{0}` but got `{1}`")]
    MismatchTokenExpectation(char, char),
}

pub type Result<T> = std::result::Result<T, TokenizerError>;
