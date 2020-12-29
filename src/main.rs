pub mod server;
use server::Server;
use ssh2::Session;
use std::net::TcpStream;

fn main() {
    let server = Server {
        hostname: String::from("172.17.0.2"),
        username: String::from("root"),
        password: String::from("pop"),
    };
    
    println!("Hello, world!");
}
