use std::io::BufReader;
use std::io::Read;
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
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut reader = BufReader::new(&mut stream);
                    let mut buffer = [0; 1024];
                    stream.read(&mut stream);
                }

                Err(e) => {
                    println!("Connection failed {}", e);
                }
            }
        }
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
//build infinite loop (while loop), in every iteration it will look for more connections OR use "loop" keyword to create infinite loop
//to name a loop: 'name:loop   where name is a label
//accept() means to look for TCP connections
//tuple example:let tup = (4, "hi", listener fixed length, different data types
//read() is not part of the TCP Stream struct, it is part of the Read trait, so we need to import it
//read() is a method that will read from the stream'
//
