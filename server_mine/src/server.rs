use std::net::TcpListener;
use crate::http::{Request, Response, StatusCode, ParseError};
use std::convert::TryFrom;
use std::io::{Read};

pub trait Handler {
    fn handle_request(&mut self, request: &Request) -> Response;
    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
    port: i32
}

impl Server {
    pub fn new(addr: String, port: i32) -> Self {
        Server {
            addr,
            port,
        }
    }

    pub fn run(self, mut handler: impl Handler) {
        println!("Server listening on {}:{}", self.addr, self.port);

        let listener = TcpListener::bind(format!("{}:{}", &self.addr, &self.port)).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, addr)) => {
                    println!("new client: {:?}", addr);
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("stream: {}", String::from_utf8_lossy(&buffer));

                            let response = match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                   handler.handle_request(&request)
                                },
                                Err(e) => {
                                    handler.handle_bad_request(&e)
                                }
                            };

                            if let Err(e) = response.send(&mut stream) {
                                println!("Failed to send response: {}", e);
                            }

                        },
                        Err(e) => println!("failed to read stream: {:?}", e),
                    }
                },
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    }
}