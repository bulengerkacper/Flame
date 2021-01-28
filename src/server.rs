extern crate ssh2;
use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;

pub struct Server {
    pub hostname: String,
    pub username: String,
    pub password: String,
}

impl Server {
    pub fn execute_command(&self, command: &str) -> String {
        let tcp = TcpStream::connect(&self.hostname).unwrap();
        let mut sess = Session::new().unwrap();
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        sess.userauth_password(&self.username, &self.password)
            .unwrap();
        let mut channel = sess.channel_session().unwrap();
        channel.exec(command).unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        channel.wait_close();
        s
    }
}

pub struct Disk {
    pub disk_name: String,
}

impl Disk {
    pub fn create_zeoring_command(&self) -> String {
        let main = String::from("cat /dev/urandom > ");
        let mut request = main.clone();
        request.push_str(&self.disk_name);
        request
    }
}
