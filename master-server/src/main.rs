use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

struct ChildServers{
    ipAddr: [u8; 4],
    port: u16,
}

fn main() {
    listen_connection()
}

fn listen_connection(){
    let PORT = 3333;
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

fn get_child_servers(){
    let contents = fs::read_to_string(filename)
}

fn check_on_files(){

}

fn send_files(mut stream: TcpStream){
    stream.write(b"test");
}
