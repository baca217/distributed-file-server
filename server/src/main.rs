use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::fs;

fn main() {
    listen_connection()
}

fn listen_connection(){
    let PORT = 5555;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT)).unwrap();
    println!("Server listening on port {}", PORT);

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    send_files(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn get_files() -> Vec<String>{
    let paths = fs::read_dir("./files").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    Vec::new()
}

fn send_files(mut stream: TcpStream){
    let str1 = "frick.txt.1".to_string();
    let str2 = "frick.txt.2".to_string();
    let msg = format!("LEN:{}\n{}\nLEN:{}\n{}", str1.len(), str1, str2.len(), str2);
    stream.write(msg.as_bytes());
}
