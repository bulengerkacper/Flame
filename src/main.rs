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
    let zeroed_commands = disks_data.replace("/dev/", "cat /dev/urandom > /dev/");
    let separated_commands = zeroed_commands.split("\n");

    for command in separated_commands {
        // println!("Execution of : {}", &command);
        let command_into_background = format!("{} &", &command);
        println!("{}", command_into_background);
        server.execute_command(&command);
    }
}
