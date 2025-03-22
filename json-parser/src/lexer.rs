use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Null,
    True,
    False,
    Number(f64),
    TString(String),
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
            Token::TString(s) => write!(f, "String({})", s),
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

pub fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let input: Vec<char> = input.trim().chars().collect();
    let mut tokens = Vec::new();

    let mut i = 0;
    while i < input.len() {
        let c = input[i];
        match c {
            '{' => tokens.push(Token::LeftBrace),
            '}' => tokens.push(Token::RightBrace),
            ':' => tokens.push(Token::Colon),
            '[' => tokens.push(Token::LeftBracket),
            ']' => tokens.push(Token::RightBracket),
            ',' => tokens.push(Token::Comma),
            '0'..='9' => i = tokenize_number(&input, &mut tokens, i)?,
            'n' => i = tokenize_null(&input, &mut tokens, i)?,
            't' => i = tokenize_true(&input, &mut tokens, i)?,
            'f' => i = tokenize_false(&input, &mut tokens, i)?,
            ' ' | '\n' | '\t' | '\r' => i = skip_whitespaces(&input, i),
            '"' => i = tokenize_string(&input, &mut tokens, i)?,
            str => return Err(format!("Unrecognised token {:}", str)),
        }
        i += 1;
    }

    Ok(tokens)
}

fn tokenize_number(input: &[char], tokens: &mut Vec<Token>, i: usize) -> Result<usize, String> {
    let mut i = i;
    let mut tmp = String::new();
    while input[i].is_ascii_digit() || input[i] == '.' {
        tmp.push(input[i]);
        i += 1;
    }
    let num = tmp.parse::<f64>().expect("It was expecting a number");
    tokens.push(Token::Number(num));
    Ok(i - 1)
}

fn tokenize_null(input: &[char], tokens: &mut Vec<Token>, i: usize) -> Result<usize, String> {
    if input[i..i + 4] == ['n', 'u', 'l', 'l'] {
        tokens.push(Token::Null);
        Ok(i + 3)
    } else {
        Err("Expected null but failed".to_string())
    }
}

fn tokenize_true(input: &[char], tokens: &mut Vec<Token>, i: usize) -> Result<usize, String> {
    if input[i..i + 4] == ['t', 'r', 'u', 'e'] {
        tokens.push(Token::True);
        Ok(i + 3)
    } else {
        Err("Expected true but failed".to_string())
    }
}

fn tokenize_false(input: &[char], tokens: &mut Vec<Token>, i: usize) -> Result<usize, String> {
    if input[i..i + 5] == ['f', 'a', 'l', 's', 'e'] {
        tokens.push(Token::False);
        Ok(i + 4)
    } else {
        Err("Expected false but failed".to_string())
    }
}

fn skip_whitespaces(input: &[char], i: usize) -> usize {
    let mut i = i;
    while i < input.len() && input[i] == ' '
        || input[i] == '\n'
        || input[i] == '\t'
        || input[i] == '\r'
    {
        i += 1;
    }
    i - 1
}

fn tokenize_string(input: &[char], tokens: &mut Vec<Token>, i: usize) -> Result<usize, String> {
    let mut i = i + 1;
    let mut str = String::new();
    while i < input.len() && input[i] != '"' {
        str.push(input[i]);
        i += 1;
    }
    if i < input.len() && input[i] == '"' {
        tokens.push(Token::TString(str));
        Ok(i)
    } else {
        Err("Expected \" but not found".to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Token, tokenize};

    #[test]
    fn tokenize_should_correctly_deal_with_known_tokens() {
        let result = tokenize("{\"foobar\": [\"apple\",\"banana\"], true false 1000.123 null}")
            .expect("should extract tokens");
        assert_eq!(
            result,
            vec!(
                Token::LeftBrace,
                Token::TString("foobar".to_string()),
                Token::Colon,
                Token::LeftBracket,
                Token::TString("apple".to_string()),
                Token::Comma,
                Token::TString("banana".to_string()),
                Token::RightBracket,
                Token::Comma,
                Token::True,
                Token::False,
                Token::Number(1000.123),
                Token::Null,
                Token::RightBrace
            )
        )
    }

    #[test]
    fn tokenize_should_correctly_deal_with_known_tokens2() {
        let result = tokenize(
            "
            {
            
            }
        ",
        )
        .expect("should extract tokens");
        assert_eq!(result, vec!(Token::LeftBrace, Token::RightBrace))
    }

    #[test]
    fn tokenize_should_correctly_deal_with_known_tokens3() {
        let result = tokenize(
            "{\"foo\": 123, \"bar\":\"spectacular\", \"baz\":null, \"b1\":true, \"b2\":false}",
        )
        .expect("should extract tokens");
        assert_eq!(
            result,
            vec!(
                Token::LeftBrace,
                Token::TString("foo".to_string()),
                Token::Colon,
                Token::Number(123.into()),
                Token::Comma,
                Token::TString("bar".to_string()),
                Token::Colon,
                Token::TString("spectacular".to_string()),
                Token::Comma,
                Token::TString("baz".to_string()),
                Token::Colon,
                Token::Null,
                Token::Comma,
                Token::TString("b1".to_string()),
                Token::Colon,
                Token::True,
                Token::Comma,
                Token::TString("b2".to_string()),
                Token::Colon,
                Token::False,
                Token::RightBrace,
            )
        )
    }

    #[test]
    fn tokenize_should_fail_if_unclosed_quote() {
        let result = tokenize("{\"foobar}").err().unwrap();
        assert!(result.contains("Expected \" but not found"))
    }

    #[test]
    fn tokenize_should_fail_if_unknown_token() {
        let result = tokenize("?").err().unwrap();
        assert!(result.contains("Unrecognised token"))
    }
}
