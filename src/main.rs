//make modules accessible: prepend name of module that contains it
use http::Method;
use http::Request;
use server::Server;
//modules I am using (tell the compiler to go look for this)

mod http;
mod server;
fn main() {
    let server = Server::new("127.0.0.1:8080".to_string()); //new is an associated function, we call it directly on struct type, not an instance of that particular sruct type, use :: to access associated functionsServer::new("127.0.0.1:8080".to_string()); //new is an associated function, we call it directly on struct type, not an instance of that particular sruct type, use :: to access associated functions
    server.run();
}

//created folder to move contents of http module's two submodules
