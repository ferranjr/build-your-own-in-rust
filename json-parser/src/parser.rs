use crate::lexer::{tokenize, Token};
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub enum JsonAST {
    JObject(Vec<(String, JsonAST)>),
    JArray(Vec<JsonAST>),
    JString(String),
    JBoolean(bool),
    JNumber(f64),
    JNull,
}

impl std::fmt::Display for JsonAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            JsonAST::JObject(elements) => write!(f, "{{ {:?} }}", elements),
            JsonAST::JArray(_) => write!(f, "[]"),
            JsonAST::JString(s) => write!(f, "\"{}\"", s),
            JsonAST::JBoolean(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            JsonAST::JNumber(n) => write!(f, "{}", n),
            JsonAST::JNull => write!(f, "null"),
        }
    }
}

fn parse_array(tokens: &mut Vec<Token>) -> Result<JsonAST, String> {
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

    Ok(JsonAST::JArray(list))
}

fn parse_json_object(tokens: &mut Vec<Token>) -> Result<JsonAST, String> {
    let mut obj: Vec<(String, JsonAST)> = Vec::new();

    loop {
        let token = tokens.pop().expect("Unexpected end for JsonObject");
        if token == Token::RightBrace {
            break;
        }
        let item_key: String;
        // We need to find key values here, so first thing is a String followed by colon
        if let Token::TString(key) = token {
            item_key = key;
        } else {
            return Err("Expected a key".to_string());
        }
        // Next should be the `:`
        let token = tokens.pop().expect("Unexpected end for JsonObject");
        if token != Token::Colon {
            return Err(format!("Expected `:` but found {}", token));
        }
        // Next should be a JsonAST
        let token = tokens.pop().expect("Unexpected end for JsonObject");
        let value = parse_json_value(tokens, token)?;
        obj.push((item_key, value));

        // Now we should check for a comma or end of jsonObject
        let token = tokens.pop().expect("Unexpected end for JsonObject");
        if token == Token::RightBrace {
            break;
        }
        if token == Token::Comma {
            continue;
        }
    }

    Ok(JsonAST::JObject(obj))
}

fn parse_json_value(tokens: &mut Vec<Token>, token: Token) -> Result<JsonAST, String> {
    match token {
        Token::Null => Ok(JsonAST::JNull),
        Token::True => Ok(JsonAST::JBoolean(true)),
        Token::False => Ok(JsonAST::JBoolean(false)),
        Token::Number(n) => Ok(JsonAST::JNumber(n)),
        Token::TString(s) => Ok(JsonAST::JString(s)),
        Token::LeftBrace => parse_json_object(tokens),
        Token::LeftBracket => parse_array(tokens),
        t => Err(format!("Unexpected token {} found", t)),
    }
}

fn parse_token_list(tokens: &mut Vec<Token>) -> Result<JsonAST, String> {
    let token = tokens.pop();

    if token == Some(Token::LeftBrace) {
        parse_json_object(tokens)
    } else if token == Some(Token::LeftBracket) {
        parse_array(tokens)
    } else {
        Err(format!(
            "Malformed JSON, expecting `{{` or `[` but got `{:?}`",
            token
        ))
    }
}

pub fn parse(input: &str) -> Result<JsonAST, String> {
    let token_result = tokenize(input)?;
    let mut tokens = token_result.into_iter().rev().collect();

    parse_token_list(&mut tokens)
}

#[cfg(test)]
mod tests {
    use crate::parser::{parse, JsonAST};

    #[test]
    fn invalid_empty_json_should_fail() {
        let result = parse("").err().unwrap();
        assert!(result.contains("Malformed JSON"))
    }

    #[test]
    fn valid_empty_json_should_succeed() {
        let result = parse("{}").unwrap();
        assert_eq!(result, JsonAST::JObject(Vec::new()))
    }

    #[test]
    fn valid_json_should_succeed_one_field() {
        let result = parse("{ \"foo\": 12}").unwrap();
        assert_eq!(
            result,
            JsonAST::JObject(vec!(("foo".to_string(), JsonAST::JNumber(12.into()))))
        )
    }

    #[test]
    fn valid_json_should_succeed_two_fields() {
        let result = parse("{\"foo\": 12,\"bar\": \"aloha\"}").unwrap();
        assert_eq!(
            result,
            JsonAST::JObject(vec!(
                ("foo".to_string(), JsonAST::JNumber(12.into())),
                ("bar".to_string(), JsonAST::JString("aloha".to_string()))
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
            JsonAST::JObject(vec!(
                ("foo".to_string(), JsonAST::JNumber(12.into())),
                ("bar".to_string(), JsonAST::JString("aloha".to_string())),
                (
                    "baz".to_string(),
                    JsonAST::JArray(vec!(
                        JsonAST::JString("foo".to_string()),
                        JsonAST::JString("bar".to_string()),
                        JsonAST::JString("baz".to_string())
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
            JsonAST::JArray(vec!(
                JsonAST::JString("foo".to_string()),
                JsonAST::JString("bar".to_string()),
                JsonAST::JString("baz".to_string())
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
            JsonAST::JObject(vec!(
                ("key".to_string(), JsonAST::JString("value".to_string())),
                ("key-n".to_string(), JsonAST::JNumber(101.into())),
                (
                    "key-o".to_string(),
                    JsonAST::JObject(vec!((
                        "inner key".to_string(),
                        JsonAST::JString("inner value".to_string())
                    )))
                ),
                (
                    "key-l".to_string(),
                    JsonAST::JArray(vec!(JsonAST::JString("list value".to_string())))
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
            JsonAST::JObject(vec!(
                ("key".to_string(), JsonAST::JString("value".to_string())),
                ("key-n".to_string(), JsonAST::JNumber(101.into())),
                ("key-o".to_string(), JsonAST::JObject(Vec::new())),
                ("key-l".to_string(), JsonAST::JArray(Vec::new()))
            ))
        )
    }
}
