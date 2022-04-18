use std::net::TcpListener;
use crate::http::{Request, Response, StatusCode};
use std::convert::TryFrom;
use std::io::{Read, Write};

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

    pub fn run(self) {
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
                                   dbg!(request);
                                   Response::new(StatusCode::Ok, Some("<h1>It works !!!</h1>".to_string()))
                                },
                                Err(e) => {
                                    println!("failed to parse request: {:?}", e);
                                    Response::new(StatusCode::BadRequest, None)
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