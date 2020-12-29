extern crate ssh2;
use ssh2::Session;

pub struct Server {
    pub hostname: String,
    pub username: String,
    pub password: String,
}

impl Server {
    fn execute_command(self, session: &mut Session) {

    }
}
