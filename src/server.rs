//use std::io::BufReader;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto; //compiler will automatically implement for the slice: Request::try_from(&buffer[..]);
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
                    // let mut reader = BufReader::new(&mut stream);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Receiver request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    println!("Request");
                                }
                                Err(e) => {
                                    println!("Failed to parse request: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            println!("Connection failed {}", e);
                        }
                    }
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
//Stream: A stream is a flow of data that is transmitted over a network
//connection, in this case, a TCP connection. The stream variable  represents
//the TCP connection itself, a continuous flow of data.
//Buffer:temporary storage area for data that is being read from or written
//to a stream.
//In other words, the stream is the source of the data, and the buffer is a
//temporary storage area for a chunk of that data.
//this: Request::try_from(&buffer as &[u8]); is equivalent to Request::try_from(&buffer[..]);
//let res: &Result<Request, _> = &buffer[..].try_into(); is now possible
