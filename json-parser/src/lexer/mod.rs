pub(crate) use crate::lexer::domain::*;

pub mod domain;

pub fn tokenize(input: &str) -> Result<Vec<Token>> {
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
            _ => return Err(TokenizerError::InvalidCharacter(c)),
        }
        i += 1;
    }

    Ok(tokens)
}

fn tokenize_number(input: &[char], tokens: &mut Vec<Token>, i: usize) -> Result<usize> {
    let mut i = i;
    let mut tmp = String::new();
    while input[i].is_ascii_digit() || input[i] == '.' {
        tmp.push(input[i]);
        i += 1;
    }
    let num = tmp.parse::<f64>()?;
    tokens.push(Token::Number(num));
    Ok(i - 1)
}

fn tokenize_null(input: &[char], tokens: &mut Vec<Token>, i: usize) -> Result<usize> {
    if input[i..i + 4] == ['n', 'u', 'l', 'l'] {
        tokens.push(Token::Null);
        Ok(i + 3)
    } else {
        Err(TokenizerError::ExpectedNull)
    }
}

fn tokenize_true(input: &[char], tokens: &mut Vec<Token>, i: usize) -> Result<usize> {
    if input[i..i + 4] == ['t', 'r', 'u', 'e'] {
        tokens.push(Token::True);
        Ok(i + 3)
    } else {
        Err(TokenizerError::ParseBooleanError(
            true,
            input[i..i + 4].iter().collect::<String>(),
        ))
    }
}

fn tokenize_false(input: &[char], tokens: &mut Vec<Token>, i: usize) -> Result<usize> {
    if input[i..i + 5] == ['f', 'a', 'l', 's', 'e'] {
        tokens.push(Token::False);
        Ok(i + 4)
    } else {
        Err(TokenizerError::ParseBooleanError(
            false,
            input[i..i + 5].iter().collect::<String>(),
        ))
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

fn tokenize_string(input: &[char], tokens: &mut Vec<Token>, i: usize) -> Result<usize> {
    let mut i = i + 1;
    let mut str = String::new();
    while i < input.len() && input[i] != '"' {
        str.push(input[i]);
        i += 1;
    }
    if i < input.len() && input[i] == '"' {
        tokens.push(Token::String(str));
        Ok(i)
    } else {
        Err(TokenizerError::MismatchTokenExpectation(
            '"',
            input[input.len() - 1],
        ))
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::{Token, TokenizerError, tokenize};

    #[test]
    fn tokenize_should_correctly_deal_with_known_tokens() {
        let result = tokenize("{\"foobar\": [\"apple\",\"banana\"], true false 1000.123 null}")
            .expect("should extract tokens");
        assert_eq!(
            result,
            vec!(
                Token::LeftBrace,
                Token::String("foobar".to_string()),
                Token::Colon,
                Token::LeftBracket,
                Token::String("apple".to_string()),
                Token::Comma,
                Token::String("banana".to_string()),
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
                Token::String("foo".to_string()),
                Token::Colon,
                Token::Number(123.into()),
                Token::Comma,
                Token::String("bar".to_string()),
                Token::Colon,
                Token::String("spectacular".to_string()),
                Token::Comma,
                Token::String("baz".to_string()),
                Token::Colon,
                Token::Null,
                Token::Comma,
                Token::String("b1".to_string()),
                Token::Colon,
                Token::True,
                Token::Comma,
                Token::String("b2".to_string()),
                Token::Colon,
                Token::False,
                Token::RightBrace,
            )
        )
    }

    #[test]
    fn tokenize_should_fail_if_unclosed_quote() {
        let error = tokenize("{\"foobar}").err().unwrap();
        assert_eq!(error, TokenizerError::MismatchTokenExpectation('"', '}'));
    }

    #[test]
    fn tokenize_should_fail_if_unknown_token() {
        let error = tokenize("?").err().unwrap();
        assert_eq!(error, TokenizerError::InvalidCharacter('?'));
    }
}
