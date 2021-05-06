use std::io::prelude::*;
use std::net::TcpStream;
use std::str::from_utf8;

fn main() {
    get_files();
}

fn get_files(){
    let mut tot = String::new();
    match TcpStream::connect("127.0.0.1:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            let mut data = [0 as u8; 8192];

            match stream.read(&mut data){
                Ok(_) => {
                    let text = from_utf8(&data).unwrap();
                    println!("RECEIVED: {}", text);
                    tot.push_str("text");
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("TOT: {}", tot);
} //stream is closed here
