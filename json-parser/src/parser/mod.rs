use domain::{ParserError, Result};

use crate::{
    lexer::{Token, tokenize},
    parser::domain::JsonAST,
};

mod domain;

fn parse_array(tokens: &mut Vec<Token>) -> Result<JsonAST> {
    let mut list: Vec<JsonAST> = Vec::new();

    loop {
        let token = tokens.pop().expect("Unexpected end for JsonArray");
        if token == Token::RightBracket {
            break;
        }
        let value = parse_json_value(tokens, token)?;

        // Should be comma or end
        let token = tokens.pop().expect("Unexpected end for JsonArray");

        list.push(value);

        if token == Token::RightBracket {
            break;
        }
        if token == Token::Comma {
            continue;
        }
    }

    Ok(JsonAST::Array(list))
}

fn parse_json_object(tokens: &mut Vec<Token>) -> Result<JsonAST> {
    let mut obj: Vec<(String, JsonAST)> = Vec::new();

    loop {
        let token = tokens.pop().ok_or(ParserError::UnexpectedEOF)?;
        if token == Token::RightBrace {
            break;
        }
        let item_key: String;
        // We need to find key values here, so first thing is a String followed by colon
        if let Token::String(key) = token {
            item_key = key;
        } else {
            return Err(ParserError::ExpectedKey(token));
        }
        // Next should be the `:`
        let token = tokens.pop().ok_or(ParserError::UnexpectedEOF)?;
        if token != Token::Colon {
            return Err(ParserError::ExpectedTokenMismatch(token, Token::Colon));
        }
        // Next should be a JsonAST
        let token = tokens.pop().ok_or(ParserError::UnexpectedEOF)?;
        let value = parse_json_value(tokens, token)?;
        obj.push((item_key, value));

        // Now we should check for a comma or end of jsonObject
        let token = tokens.pop().ok_or(ParserError::UnexpectedEOF)?;
        if token == Token::RightBrace {
            break;
        }
        if token == Token::Comma {
            continue;
        }
    }

    Ok(JsonAST::Object(obj))
}

fn parse_json_value(tokens: &mut Vec<Token>, token: Token) -> Result<JsonAST> {
    match token {
        Token::Null => Ok(JsonAST::Null),
        Token::True => Ok(JsonAST::Boolean(true)),
        Token::False => Ok(JsonAST::Boolean(false)),
        Token::Number(n) => Ok(JsonAST::Number(n)),
        Token::String(s) => Ok(JsonAST::String(s)),
        Token::LeftBrace => parse_json_object(tokens),
        Token::LeftBracket => parse_array(tokens),
        t => Err(ParserError::UnexpectedToken(t)),
    }
}

fn parse_token_list(tokens: &mut Vec<Token>) -> Result<JsonAST> {
    let token = tokens.pop();

    if token == Some(Token::LeftBrace) {
        parse_json_object(tokens)
    } else if token == Some(Token::LeftBracket) {
        parse_array(tokens)
    } else if let Some(t) = token {
        Err(ParserError::UnexpectedToken(t))
    } else {
        Err(ParserError::UnexpectedEOF)
    }
}

pub fn parse(input: &str) -> Result<JsonAST> {
    let token_result = tokenize(input)?;

    let mut tokens = token_result.into_iter().rev().collect();

    let result = parse_token_list(&mut tokens);

    // If there are other tokens we should fail as it is a malformed json
    if let Some(t) = tokens.pop() {
        Err(ParserError::UnexpectedTokenAfterEOF(t))
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lexer::Token,
        parser::{JsonAST, domain::ParserError, parse},
    };

    #[test]
    fn invalid_empty_json_should_fail() {
        let result = parse("").err().unwrap();
        assert_eq!(result, ParserError::UnexpectedEOF);
    }

    #[test]
    fn valid_empty_json_should_succeed() {
        let result = parse("{}").unwrap();
        assert_eq!(result, JsonAST::Object(Vec::new()))
    }

    #[test]
    fn valid_json_should_succeed_one_field() {
        let result = parse("{ \"foo\": 12}").unwrap();
        assert_eq!(
            result,
            JsonAST::Object(vec!(("foo".to_string(), JsonAST::Number(12.into()))))
        )
    }

    #[test]
    fn valid_json_should_succeed_two_fields() {
        let result = parse("{\"foo\": 12,\"bar\": \"aloha\"}").unwrap();
        assert_eq!(
            result,
            JsonAST::Object(vec!(
                ("foo".to_string(), JsonAST::Number(12.into())),
                ("bar".to_string(), JsonAST::String("aloha".to_string()))
            ))
        )
    }

    #[test]
    fn valid_json_should_succeed_including_list_of_strings() {
        let result =
            parse("{\"foo\": 12,\"bar\": \"aloha\", \"baz\": [ \"foo\", \"bar\", \"baz\"]}")
                .unwrap();
        assert_eq!(
            result,
            JsonAST::Object(vec!(
                ("foo".to_string(), JsonAST::Number(12.into())),
                ("bar".to_string(), JsonAST::String("aloha".to_string())),
                (
                    "baz".to_string(),
                    JsonAST::Array(vec!(
                        JsonAST::String("foo".to_string()),
                        JsonAST::String("bar".to_string()),
                        JsonAST::String("baz".to_string())
                    ))
                ),
            ))
        )
    }

    #[test]
    fn valid_json_should_succeed_when_directly_a_list_of_strings() {
        let result = parse("[ \"foo\", \"bar\", \"baz\"]").unwrap();
        assert_eq!(
            result,
            JsonAST::Array(vec!(
                JsonAST::String("foo".to_string()),
                JsonAST::String("bar".to_string()),
                JsonAST::String("baz".to_string())
            ))
        )
    }

    #[test]
    fn valid_2_json_step_4_should_succeed() {
        let result = parse(
            "
            {
                \"key\": \"value\",
                \"key-n\": 101,
                \"key-o\": {
                    \"inner key\": \"inner value\"
                },
                \"key-l\": [\"list value\"]
            }
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            JsonAST::Object(vec!(
                ("key".to_string(), JsonAST::String("value".to_string())),
                ("key-n".to_string(), JsonAST::Number(101.into())),
                (
                    "key-o".to_string(),
                    JsonAST::Object(vec!((
                        "inner key".to_string(),
                        JsonAST::String("inner value".to_string())
                    )))
                ),
                (
                    "key-l".to_string(),
                    JsonAST::Array(vec!(JsonAST::String("list value".to_string())))
                )
            ))
        )
    }
    #[test]
    fn valid_json_step_4_should_succeed() {
        let result = parse(
            "
            {
                \"key\": \"value\",
                \"key-n\": 101,
                \"key-o\": {},
                \"key-l\": []
            }
        ",
        )
        .unwrap();
        assert_eq!(
            result,
            JsonAST::Object(vec!(
                ("key".to_string(), JsonAST::String("value".to_string())),
                ("key-n".to_string(), JsonAST::Number(101.into())),
                ("key-o".to_string(), JsonAST::Object(Vec::new())),
                ("key-l".to_string(), JsonAST::Array(Vec::new()))
            ))
        )
    }

    #[test]
    fn valid_json_step_4_followed_by_more_input_should_fail() {
        let result = parse(
            "
            {
                \"key\": \"value\",
                \"key-n\": 101,
                \"key-o\": {},
                \"key-l\": []
            } \"aloha\"
        ",
        )
        .err()
        .unwrap();
        assert_eq!(
            result,
            ParserError::UnexpectedTokenAfterEOF(Token::String("aloha".to_string()))
        )
    }

    #[test]
    fn start_of_json_and_end_of_input_should_fail() {
        let result = parse(
            "
            {
        ",
        )
        .err()
        .unwrap();
        assert_eq!(result, ParserError::UnexpectedEOF)
    }
}
