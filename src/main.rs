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

    let disks_data = server.execute_command("df -h --output=source | grep dev/");
    let mut disks = Vec::new();

    for disk_name in disks_data.split(" ") {
        let disk = Disk {
            disk_name: disk_name.to_string(),
        };
        disks.push(disk)
    }

    let command_creation =
        |disk: Disk| -> String { disk.create_zeoring_command("cat /dev/urandom >") };

    for disk in disks {
        let command = command_creation(disk);
        println!("{}", command);
    }
}
