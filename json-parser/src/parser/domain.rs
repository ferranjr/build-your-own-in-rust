use crate::lexer::{Token, TokenizerError};
use std::fmt::Formatter;
use tracing::error;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonAST {
    Object(Vec<(String, JsonAST)>),
    Array(Vec<JsonAST>),
    String(String),
    Boolean(bool),
    Number(f64),
    Null,
}

impl std::fmt::Display for JsonAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonAST::Object(elements) => write!(f, "{{ {:?} }}", elements),
            JsonAST::Array(_) => write!(f, "[]"),
            JsonAST::String(s) => write!(f, "\"{}\"", s),
            JsonAST::Boolean(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            JsonAST::Number(n) => write!(f, "{}", n),
            JsonAST::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ParserError {
    #[error("Expected token {0} but go {1} instead")]
    ExpectedTokenMismatch(Token, Token),
    #[error("Unexpected token {0}")]
    UnexpectedToken(Token),
    #[error("Expected a json key found {0}")]
    ExpectedKey(Token),
    #[error("Unexpected end of file")]
    UnexpectedEOF,
    #[error("Unexpected token {0} after end of file")]
    UnexpectedTokenAfterEOF(Token),
    #[error("Unable to tokenise the string")]
    TokenisingError(#[from] TokenizerError),
}

pub(crate) type Result<T> = std::result::Result<T, ParserError>;
