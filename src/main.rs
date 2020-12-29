pub mod server;
use server::Server;

use ssh2::Session;
use std::net::TcpStream;
use std::io::Read;

fn main() {
    let server = Server {
        hostname: String::from("172.17.0.2"),
        username: String::from("root"),
        password: String::from("pop"),
    };

    let hostname = &server.hostname[..];
    let username = &server.username[..];
    let password = &server.password[..];

    let tcp = TcpStream::connect(&hostname).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();

    sess.userauth_password(&username, &password).unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());

    assert!(sess.authenticated());
}
