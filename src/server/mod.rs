pub mod request;
pub mod response;
use request::Request;
use response::Response;
use std::collections::HashMap;
use std::io::{BufRead, BufReader, Read};
use std::net::TcpListener;
use std::net::TcpStream;
#[macro_use]
mod custom_macros;

enum RequestMethod {
    GET,
    POST,
}
struct Route {
    path: String,
    handler: Box<dyn Fn(Request, Response) -> ()>,
}

impl Route {
    fn call(&self, req: Request, res: Response) {
        (self.handler)(req, res)
    }
}

pub struct Server {
    handlers: HashMap<String, Route>,
}

impl Server {
    pub fn new() -> Server {
        Server {
            handlers: HashMap::new(),
        }
    }

    pub fn get<T>(&mut self, path: &str, route_handler: T)
    where
        T: Fn(Request, Response) -> () + 'static,
    {
        self.handlers.insert(
            path.to_string(),
            Route {
                path: path.to_string(),
                handler: Box::from(route_handler),
            },
        );
    }

    fn build_response(&mut self, stream: TcpStream) -> Response {
        Response::new(stream)
    }

    fn build_request(&mut self, stream: &TcpStream) -> Request {
        let mut reader = BufReader::new(stream);

        let mut path = String::new();
        let mut method = String::new();

        for line in reader.by_ref().lines() {
            let line: String = line.unwrap();
            if line == "" {
                break;
            }

            if line.starts_with("GET") {
                for (index, word) in line.split(" ").enumerate() {
                    match index {
                        0 => method = String::from(word),
                        1 => path = String::from(word),
                        _ => (),
                    }
                }
            }
        }

        println!("What");
        Request {
            path: path,
            method: method,
        }
    }

    fn route_request(&mut self, connection: TcpStream) {
        let request = self.build_request(&connection);
        let response = self.build_response(connection);

        let path = &request.path;

        println!("{}", path);

        let selected_route = &self.handlers.get(path);
        if selected_route.is_some() {
            let selected_route: &Route = selected_route.unwrap();
            &selected_route.call(request, response);
        }
    }

    pub fn listen<T>(&mut self, port: u16, callback: T)
    where
        T: Fn() -> (),
    {
        let address = format!("127.0.0.1:{}", port);

        let port_listener = TcpListener::bind(address).unwrap();

        callback();

        for connection in port_listener.incoming() {
            let connection = connection.unwrap();
            self.route_request(connection);
        }
    }
}
