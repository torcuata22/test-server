use std::net::TcpListener;
pub struct Server {
    addr: String,
}
impl Server {
    pub fn new(addr: String) -> Self {
        //returns a new instance of Server
        Self { addr }
    }

    pub fn run(self) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
    }
}

//NOTES:
//to add functionality, write implementation block
//Every file is treated as a module (no need for mod keyworkd)
//Server is a  struct, a data type that lets you group together related data
//created server module (everything private by default, need to state which parts are publlic with pub)
//created a struct
//to start listening for TCP, use the net module from standard library (look it up in rust-lang.org)
//unwrap() is a method that will panic if the operation fails (terminates program and logs eror)
