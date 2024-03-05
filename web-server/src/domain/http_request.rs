use nom::bytes::complete::{take_until};
use nom::character::complete::{alphanumeric1, space0};
use nom::error::{context, VerboseError};
use nom::sequence::tuple;
use nom::IResult;
use std::str::FromStr;

/// On this implementation we only care about this two
/// parameters of the request, which come on the very first line
#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub path: String,
}

impl FromStr for HttpRequest {
    type Err = std::io::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_http_request(s)
            .map(|(_, http_request)| http_request)
            .map_err(|_| std::io::Error::from(std::io::ErrorKind::InvalidData))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    CONNECT,
    GET,
    HEAD,
    DELETE,
    OPTIONS,
    PATCH,
    POST,
    PUT,
    TRACE,
}

impl FromStr for Method {
    type Err = std::io::Error;

    fn from_str(s: &str) -> std::io::Result<Self> {
        match s.to_uppercase().as_str() {
            "CONNECT" => Ok(Method::CONNECT),
            "GET" => Ok(Method::GET),
            "HEAD" => Ok(Method::HEAD),
            "DELETE" => Ok(Method::DELETE),
            "OPTIONS" => Ok(Method::OPTIONS),
            "PATCH" => Ok(Method::PATCH),
            "POST" => Ok(Method::POST),
            "PUT" => Ok(Method::PUT),
            "TRACE" => Ok(Method::TRACE),
            _ => Err(std::io::Error::from(std::io::ErrorKind::InvalidData)),
        }
    }
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

fn parse_method(input: &str) -> Res<&str, Method> {
    nom::combinator::map_res(alphanumeric1, FromStr::from_str)(input)
}

fn parse_path(input: &str) -> Res<&str, String> {
    context("path", take_until(" "))(input).map(|(next, res)| (next, res.to_string()))
}

fn parse_http_request(input: &str) -> Res<&str, HttpRequest> {
    context("http_request", tuple((parse_method, space0, parse_path)))(input).map(
        |(next, (method, _, path))| {
            (
                next,
                HttpRequest {
                    method,
                    path: path.to_string(),
                },
            )
        },
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use nom::error::{ErrorKind, VerboseErrorKind};

    #[test]
    fn parser_method_works_when_valid_method_input() {
        assert_eq!(parse_method("CONNECT"), Ok(("", Method::CONNECT)));
        assert_eq!(parse_method("GET"), Ok(("", Method::GET)));
        assert_eq!(parse_method("HEAD"), Ok(("", Method::HEAD)));
        assert_eq!(parse_method("DELETE"), Ok(("", Method::DELETE)));
        assert_eq!(parse_method("OPTIONS"), Ok(("", Method::OPTIONS)));
        assert_eq!(parse_method("PATCH"), Ok(("", Method::PATCH)));
        assert_eq!(parse_method("POST"), Ok(("", Method::POST)));
        assert_eq!(parse_method("PUT"), Ok(("", Method::PUT)));
        assert_eq!(parse_method("TRACE"), Ok(("", Method::TRACE)));
        assert_eq!(parse_method("connect"), Ok(("", Method::CONNECT)));
        assert_eq!(parse_method("get"), Ok(("", Method::GET)));
        assert_eq!(parse_method("head"), Ok(("", Method::HEAD)));
        assert_eq!(parse_method("delete"), Ok(("", Method::DELETE)));
        assert_eq!(parse_method("options"), Ok(("", Method::OPTIONS)));
        assert_eq!(parse_method("patch"), Ok(("", Method::PATCH)));
        assert_eq!(parse_method("post"), Ok(("", Method::POST)));
        assert_eq!(parse_method("put"), Ok(("", Method::PUT)));
        assert_eq!(parse_method("trace"), Ok(("", Method::TRACE)));
    }

    #[test]
    fn parser_method_fails_for_invalid_input() {
        assert_eq!(
            parse_method("banana"),
            Err(nom::Err::Error(VerboseError {
                errors: vec![("banana", VerboseErrorKind::Nom(ErrorKind::MapRes))]
            }))
        );
    }

    #[test]
    fn parser_http_requests_works_for_valid_inputs() {
        let input = "GET /aloha HTTP/1.1";
        assert_eq!(
            parse_http_request(input),
            Ok((
                " HTTP/1.1",
                HttpRequest {
                    method: Method::GET,
                    path: "/aloha".to_string(),
                }
            ))
        )
    }

    #[test]
    fn method_creation_succeeds_from_valid_string() {
        let valid_strings = vec![
            ("CONNECT", Method::CONNECT),
            ("GET", Method::GET),
            ("HEAD", Method::HEAD),
            ("DELETE", Method::DELETE),
            ("OPTIONS", Method::OPTIONS),
            ("PATCH", Method::PATCH),
            ("POST", Method::POST),
            ("PUT", Method::PUT),
            ("TRACE", Method::TRACE),
        ];

        for (str, expected) in valid_strings.iter() {
            let method: Method = str
                .parse::<Method>()
                .expect("Failed to read valid string as Method");
            assert_eq!(method, *expected);
        }
    }

    #[test]
    fn method_creation_fails_from_invalid_string() {
        assert!("INVALID".parse::<Method>().is_err());
    }

    #[test]
    fn http_requests_succeeds_from_valid_string() {
        let http_request_line = "GET /aloha HTTP/1.1";
        let http_request = http_request_line
            .parse::<HttpRequest>()
            .expect("Failed to extract the request info.");
        let expected = HttpRequest {
            method: Method::GET,
            path: "/aloha".to_string(),
        };
        assert_eq!(http_request, expected);
    }
}
