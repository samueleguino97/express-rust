extern crate web_server;
use web_server::server::request::Request;
use web_server::server::response::Response;
use web_server::server::Server;

fn main() {
    let mut server = Server::new();

    server.get("/lol", |_: Request, _: Response| {
        println!("It actually works")
    });
    server.get("/samuel", |_: Request, mut res: Response| {
        res.status(400).send("<h1>sa</h1>")
    });

    server.listen(5000, || println!("Server Listening at: {}", 5000));
}
