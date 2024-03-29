use std::fmt::Display;

#[derive(Debug)]
pub struct HttpResponse {
    status_code: StatusCodes,
    content: String,
}

impl HttpResponse {
    pub fn new(status_code: StatusCodes, content: Option<String>) -> Self {
        HttpResponse {
            status_code,
            content: content.unwrap_or("".to_string()),
        }
    }

    pub fn response_string(&self) -> String {
        let content_length_line = format!("Content-Length: {}", self.content.len());
        format!(
            "HTTP/1.1 {}\r\n{}\r\n\r\n{}\r\n",
            self.status_code, content_length_line, self.content
        )
    }
}

#[derive(Debug)]
pub enum StatusCodes {
    OK,
    Created,
    Accepted,
    NoContent,
    NotFound,
    InternalServerError,
}

impl Display for StatusCodes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            StatusCodes::OK => "200 OK",
            StatusCodes::Created => "201 Created",
            StatusCodes::Accepted => "202 Accepted",
            StatusCodes::NoContent => "204 No Content",
            StatusCodes::NotFound => "404 Not Found",
            StatusCodes::InternalServerError => "500 Internal Server Error",
        };
        write!(f, "{}", str)
    }
}

#[cfg(test)]
mod tests {
    use super::{HttpResponse, StatusCodes};

    #[test]
    fn http_response_builds_ok_string() {
        let str = HttpResponse::new(StatusCodes::OK, Some("Requested path: /foobar".to_string()))
            .response_string();
        assert_eq!(
            str,
            "HTTP/1.1 200 OK\r\nContent-Length: 23\r\n\r\nRequested path: /foobar\r\n"
        )
    }

    #[test]
    fn http_response_builds_created_string() {
        let str = HttpResponse::new(
            StatusCodes::Created,
            Some("Requested path: /foobar".to_string()),
        )
        .response_string();
        assert_eq!(
            str,
            "HTTP/1.1 201 Created\r\nContent-Length: 23\r\n\r\nRequested path: /foobar\r\n"
        )
    }

    #[test]
    fn http_response_builds_accepted_string() {
        let str = HttpResponse::new(
            StatusCodes::Accepted,
            Some("Requested path: /foobar".to_string()),
        )
        .response_string();
        assert_eq!(
            str,
            "HTTP/1.1 202 Accepted\r\nContent-Length: 23\r\n\r\nRequested path: /foobar\r\n"
        )
    }

    #[test]
    fn http_response_builds_no_content_string() {
        let str = HttpResponse::new(StatusCodes::NoContent, None).response_string();
        assert_eq!(
            str,
            "HTTP/1.1 204 No Content\r\nContent-Length: 0\r\n\r\n\r\n"
        )
    }

    #[test]
    fn http_response_builds_not_found_string() {
        let str = HttpResponse::new(StatusCodes::NotFound, None).response_string();
        assert_eq!(
            str,
            "HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n\r\n"
        )
    }

    #[test]
    fn http_response_builds_internal_server_error_string() {
        let str = HttpResponse::new(StatusCodes::InternalServerError, None).response_string();
        assert_eq!(
            str,
            "HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\n\r\n\r\n"
        )
    }
}
