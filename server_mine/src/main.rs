

use server::Server;
use http::Request;
use http::Method;

mod http;
mod server;

fn main() {
    //let get = Method::GET("abcd".to_string());
    //let put = Method::PUT;
    //let patch = Method::PATCH;
    //let get = Method::DELETE(100);


    let server = Server::new("127.0.0.1".to_string(), 8081);
    server.run()
}

