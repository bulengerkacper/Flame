pub mod server;
use server::Server;

use std::env;

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
    let zeroing_command = String::from("cat /dev/urandom > ");
    let disks = server.execute_command("df -h --output=source | grep dev/");
    for disk_name in disks.split(" ") {
        let cmd = "cat /dev/urandom > ".to_string();
        let cmd2 = format!("{} {}",cmd,disk_name);
        println!("{}",cmd2);
    }  
}
