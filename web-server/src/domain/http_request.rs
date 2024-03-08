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
        let bits = s.split(' ').collect::<Vec<&str>>();
        let method = bits[0].parse::<Method>()?;
        let path = bits[1];
        Ok(HttpRequest {
            method,
            path: path.to_string(),
        })
    }
}

#[derive(Debug, PartialEq)]
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
        match s {
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

#[cfg(test)]
mod test {
    use super::*;

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
