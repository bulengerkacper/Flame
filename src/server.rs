struct Server {
    hostname: String,
    password: String,
}

impl Server {
    pub fn new(hostname: String, password: String) -> Server {
        Server {
            hostname: hostname,
            password: password,
        }
    }
}
