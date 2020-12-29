struct Server {
    hostname: String,
    username: String,
    password: String,
}

impl Server {
    pub fn new(hostname: String,username:String, password: String) -> Server {
        Server {
            hostname: hostname,
            username: username,
            password: password,
        }
    }

}
