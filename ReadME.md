# Express in Rust

## Purpose

My purpose for this project was to implement a Web Server from scratch while mantaining the ExpressJS API


## Code Example 

```rust
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