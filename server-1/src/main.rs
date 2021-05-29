use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::fs;
//extern crate glob;
//use glob::glob;

fn main() {
    listen_connection();
}

fn listen_connection(){
    let PORT = match get_port(){
        Some(v) => v,
        None => return,
    };
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

fn get_port() -> Option<i32>{
    let mut file = match fs::File::open("./info/port.txt"){
        Ok(v) => v,
        Err(e) => {
            println!("ERR: {}", e);
            return None;
        },
    };
    let mut contents = String::new();
    match file.read_to_string(&mut contents){
        Ok(v) => (),
        Err(e) => {
            println!("couldn't read info from port");
            return None;
        }
    };
    println!("CONTENT: {}", contents);
    match contents.trim_end().parse::<i32>(){
        Ok(v) => return Some(v),
        Err(e) => {
            println!("ERR: {}", e);
            return None;
        },
    }
}

fn handler(mut stream: TcpStream){
    let CHUNK = 1024
    let mut buffer: [u8; CHUNK] = [0; CHUNK];

    stream.read(&mut buffer);

}

fn send_files(mut stream: TcpStream) -> Result<(), std::io::Error>{
    let mut files:String = String::new();

    let paths = match fs::read_dir("./files"){
        Ok(v) => v,
        Err(e) => {
            println!("Couldn't open directory ./files");
            return Err(e);
        },
    };

    for path in paths {
        let temp:String = match path.unwrap().path().into_os_string().into_string(){
            Ok(v) => v,
            Err(e) => {
                println!("Wasn't able to convert path to string");
                continue;
            },
        };
        let vec: Vec<&str> = temp.split("/").collect();
        let file: &str = vec[vec.len()-1];
        files.push_str(&file.len().to_string());
        files.push_str("\n");
        files.push_str(vec[vec.len()-1]);
        files.push_str("\n");
    }

    stream.write(files.as_bytes());
    println!("sent files");
    Ok(())
}
