pub mod server;
use server::Server;

use ssh2::Session;
use std::net::TcpStream;
use std::io::Read;



fn main() {
    let server = Server {
        hostname: String::from("0.0.0.0:49154"),
        username: String::from("root"),
        password: String::from("root"),
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
    channel.exec("ps").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());

}
