use std::io::prelude::*;
use std::net::TcpStream;
use std::str::from_utf8;
use std::fs; //for pulling server information from file
use std::net::{SocketAddr, IpAddr, Ipv4Addr}; //for checking the IP addresses within the file

fn main() {
    get_child_servers();
}

/*
 * Function: get_files()
 * Arguments: None
 * How it works: goes through a vector of child servers and sees what pieces they have. Each file
 * is assumed to have 4 pieces. If there is enough pieces to make up a whole file then this will
 * be printed to the user. If the file is incomplete an ERROR not enough pieces will be printed.
 * */
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

fn get_child_servers() -> Option<Vec<SocketAddr>>{
    let filename = "servers.txt";
    let mut servers = Vec::new();
    let content = match fs::read_to_string(filename) {
        Err(_e) => return None,
        Ok(cont) => cont,
    };
    let info = content.split("\n");
    for i in info{
        let server: SocketAddr = match i.parse(){
            Err(e) => {
                println!("ERR: {}\nVAL: {}", e, i);
                continue;
            },
            Ok(addr) => addr
        };
        println!("{:?}", server);
        servers.push(server);
    }
    return Some(servers);
}

fn check_on_files(){

}

fn send_files(mut stream: TcpStream){
    stream.write(b"test");
}

#[test]
fn test_get_servers() {
    let servs = vec![
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5555),
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6666),
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 7777),
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8888),
    ];
    let test = match get_child_servers(){
        Some(val) => val,
        None => Vec::new(),
    };
    assert_eq!(test, servs);
}
