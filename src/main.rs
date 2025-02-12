fn main() {
    let server = Server::new("127.0.0.1:8080".to_string()); //new is an associated function, we call it directly on struct type, not an instance of that particular sruct type, use :: to access associated functions
    server.run();
}

//Seerver is a  struct, a data type that lets you group together related data
//create a struct:
struct Server {
    addr: String,
}
//to add functionality, write implementation block:
impl Server {
    fn new(addr: String) -> Self {
        //returns a new instance of Server
        Self { addr }
    }

    fn run(self) {
        println!("Listening on {}", self.addr);
    }
}
