extern crate ssh2;
use ssh2::{Channel, Session};
use std::io::Read;

pub struct Server {
    pub hostname: String,
    pub username: String,
    pub password: String,
}

impl Server {
    fn execute_command(self, sess: &mut Session, chanel: &mut Channel) {
        let mut channel = sess.channel_session().unwrap();
        channel.exec("ls").unwrap();
        let mut s = String::new();
        channel.read_to_string(&mut s).unwrap();
        println!("{}", s);
        channel.wait_close();
        println!("{}", channel.exit_status().unwrap());
    }
}
