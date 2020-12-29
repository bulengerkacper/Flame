pub mod server;
use server::Server;

use std::net::TcpStream;
use ssh2::Session;

fn main() {
    let server = Server {
        hostname: String::from("172.17.0.2"),
        username: String::from("root"),
        password: String::from("pop"),
    };

    println!("Hello, world!");
    let hostname = &server.hostname[..];
    let username = &server.username[..];
    let password = &server.password[..];

    let tcp = TcpStream::connect(&hostname).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    
    sess.userauth_password(&username,&password).unwrap();
    assert!(sess.authenticated());
}

