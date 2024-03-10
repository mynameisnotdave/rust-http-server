fn main() {
    let addr = String::from("127.0.0.1:8080");
    let server = Server::new(addr);
    //server.run();
}

struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Server {
        Server {
            addr: addr
        }
    }

    fn run(self) {

    }
}