#![allow(dead_code)]

use server::Server;
use web_handler::WebHandler;
use std::env;

mod http;
mod server;
mod web_handler;

fn main() {
    //let get = Method::GET("abcd".to_string());
    //let put = Method::PUT;
    //let patch = Method::PATCH;
    //let get = Method::DELETE(100);

    let default_path = format!("{}/public", env!("CARGO_MANIFEST_DIR"));
    let public_path = env::var("PUBLIC_PATH").unwrap_or(default_path);
    println!("public_path: {}", public_path);
    let server = Server::new("127.0.0.1".to_string(), 8081);
    server.run(WebHandler::new(public_path));
}

