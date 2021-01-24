pub mod server;
use server::Server;

use ssh2::Session;
use std::env;
use std::io::Read;


fn main() {
    let args: Vec<String> = env::args().collect();

    let hostname = &args[1]; //x.x.x.x:port
    let username = &args[2];
    let password = &args[3];

    let server = Server {
        hostname: String::from(hostname),
        username: String::from(username),
        password: String::from(password),
    };

    server.execute_command("ls -al")
}
