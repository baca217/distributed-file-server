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
    let PORT = 5555;
    let listener = TcpListener::bind(format!("0.0.0.0:{}", PORT)).unwrap();
    println!("Server listening on port {}", PORT);

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let files = match get_files(){
                    Some(v) => v,
                    None => "NO FILES".to_string(),
                };
                thread::spawn(move|| {
                    send_files(stream, files)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn get_files() -> Option<String>{
    let mut holder:String = String::new();

    let paths = match fs::read_dir("./files"){
        Ok(v) => v,
        Err(E) => {
            println!("Couldn't open directory ./files");
            return None
        },
    };

    for path in paths {
        let temp:String = match path.unwrap().path().into_os_string().into_string(){
            Ok(v) => v,
            Err(e) => {
                println!("Wasn't able to conver path to string");
                continue;
            },
        };
        let vec: Vec<&str> = temp.split("/").collect();
        let file: &str = vec[vec.len()-1];
        holder.push_str(&file.len().to_string());
        holder.push_str("\n");
        holder.push_str(vec[vec.len()-1]);
        holder.push_str("\n");
    }

    println!("holder: {}", holder);
    Some(holder)
}

fn send_files(mut stream: TcpStream, files: String){
    stream.write(files.as_bytes());
}
