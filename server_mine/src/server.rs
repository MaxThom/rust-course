use std::net::TcpListener;
use crate::http::Request;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::io::Read;

pub struct Server {
    addr: String,
    port: i32
}

fn arr(a: &[u8]) {}

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

                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {},
                                Err(e) => println!("failed to parse request: {:?}", e)
                            }
                            let res: &Result<Request, _> = &buffer[..].try_into();
                        },
                        Err(e) => println!("failed to read stream: {:?}", e),
                    }
                },
                Err(e) => println!("couldn't get client: {:?}", e),
            }
        }
    }
}