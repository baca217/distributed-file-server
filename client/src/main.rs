use std::io::prelude::*;
use std::net::TcpStream;
use std::str::from_utf8;
use std::fs; //for pulling server information from file
use std::net::{SocketAddr, IpAddr, Ipv4Addr}; //for checking the IP addresses within the file
use std::collections::HashMap; //for keeping track of the files we have

struct file_info {
    servers: array[SocketAddr; 4],
}

fn main() {
    let servers = match get_child_servers(){
        None => Vec::new(),
        Some(val) => val,
    };
    get_files(servers);
}

/*
 * Function: get_files()
 *
 * Arguments: None
 *
 * How it works: goes through a vector of child servers and sees what pieces they have. Each file
 * is assumed to have 4 pieces. If there is enough pieces to make up a whole file then this will
 * be printed to the user. If the file is incomplete an ERROR not enough pieces will be printed.
 * */
fn get_files(servers: Vec<SocketAddr>){
    let mut files = Vec::new();
    for serv in servers{
        match TcpStream::connect(serv) {
            Ok(mut stream) => {
                println!("Successfully connected to server in port 3333");
                let mut data = [0 as u8; 8192];

                match stream.read(&mut data){
                    Ok(_) => {
                        let text = from_utf8(&data).unwrap().to_string();

                        println!("RECEIVED: {}", text);
                        tot.push(text);
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
    }

    for i in tot{
        println!("{}", i);
    }
} //stream is closed here

/*
 * Function: parse_avl_files
 *
 * Arguments: files: String - should be an unparased string that contains file that a server should
 * have, seperated with a newline for each file
 *
 * How it works: seperates the text by newline, then takes the last char in the string to see what
 * piece of the file it is. A vector of tuples with the file name and file number is returned.
 * */
fn parse_avl_files(files : String) -> Vec<String>{
}

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

#[test]
fn test_avl_files() {
    let servs = vec![
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 5555),
    ];

    let vals = vec![
        "file-1.txt ERROR",
    ];

    let test = match get_files(servs){
        None => Vec::new(),
        Some(val) => val,
    };

    assert_eq!(test, vals);
}
