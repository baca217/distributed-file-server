use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    get_files();
}

fn get_files() -> std::io::Result<(), std::io::Error> {
    match TcpStream::connect("127.0.0.1:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
        }
    }
} //stream is closed here
