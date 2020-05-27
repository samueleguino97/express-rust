use std::io::Write;
use std::net::TcpStream;

pub struct Response {
    connection: TcpStream,
    status: u16,
}

impl Response {
    pub fn new(connection: TcpStream) -> Response {
        Response {
            connection,
            status: 200,
        }
    }

    pub fn status(&mut self, status: u16) -> &mut Self {
        self.status = status;
        self
    }

    pub fn send(&mut self, content: &str) {
        let protocol = "HTTP/1.1";
        let code = match self.status {
            400 => "400 Bad Request",
            _ => "200 OK",
        };

        let real_content = match self.status {
            400 => "Bad Request",
            _ => content,
        };

        let response = format!("{} {}\r\n\r\n{}", protocol, code, real_content);
        self.connection.write(response.as_bytes()).unwrap();
        self.connection.flush().unwrap();
    }
}
