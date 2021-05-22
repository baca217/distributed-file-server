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
