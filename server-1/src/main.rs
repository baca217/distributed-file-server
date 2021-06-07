use std::thread;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::fs;
use std::str; //for parsing TCP stream

fn main() {
    listen_connection();
}

fn listen_connection(){
    let port = match get_port(){
        Some(v) => v,
        None => return,
    };
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port)).unwrap();
    println!("Server listening on port {}", port);

    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                handler(stream);
                /*
                thread::spawn(move|| {
                    send_files(stream)
                });
                */
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
        Ok(_v) => (),
        Err(e) => {
            println!("couldn't read info from port");
            println!("ERR: {}", e);
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

/*
 * Function Name: handler()
 *
 * Arguments: mut stream: TcpStream
 *
 * Functionality: takes a TcpStream connection as an argument. Meant to receive data from client
 * side, parse the data for an argument, and then send back the appropriate response, and do the
 * action that is wanted.
 * */
fn handler(mut stream: TcpStream){
    const CHUNK:usize = 1024;
    let mut buffer: [u8; CHUNK] = [0; CHUNK];

    loop{
        match stream.read(&mut buffer){
            Ok(_) => break,
            Err(e) => {
                println!("Error in reading TCP stream\nERR: {}", e);
                return;
            },
        }
    }

    let result = match str::from_utf8(&buffer[0..4]){
        Ok(v) => v,
        Err(e) => {
            println!("Error in converting TCP stream to string\nERR: {}", e);
            return;
        },
    };

    match result{
        "list" => {
            println!("going to send list of files to client");
            send_files(stream);
        },
        "file" => {
            println!("going to receive file from client");
            let mut fname = String::new();
            let mut loc = 4;
            loop{
                if buffer[loc] as char == '\n'{break}
                fname.push(buffer[loc] as char);
                loc += 1;
            }
            println!("fname: {}", fname);
            let mut file = match fs::File::create(fname.clone()){
                Ok(v) => v,
                Err(e) => {
                    println!("couldn't open file {}", fname);
                    println!("ERR: {}", e);
                    return
                },
            };
            buffer.iter_mut().for_each(|m| *m=0);
            match stream.read(&mut buffer){
                Ok(_) => (),
                Err(e) => {
                    println!("Error in reading TCP stream\nERR: {}", e);
                    return;
                },
            }
            file.write_all(&buffer);
        },
        "delt" => println!("going to delete the file on server"),
        _ => println!("NOT A CASE: {}", result),
    }
}

fn send_files(mut stream: TcpStream){
    let mut files:String = String::new();

    let paths = match fs::read_dir("./files"){
        Ok(v) => v,
        Err(e) => {
            println!("Couldn't open directory ./files");
            println!("ERR: {:?}", e);
            return;
        },
    };

    for path in paths {
        let temp:String = match path.unwrap().path().into_os_string().into_string(){
            Ok(v) => v,
            Err(e) => {
                println!("Wasn't able to convert path to string");
                println!("ERR: {:?}", e);
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

    match stream.write(files.as_bytes()){
        Ok(_v) => println!("send list of files to client"),
        Err(e) => {
            println!("Error occurred while trying to send list of files to client");
            println!("ERR: {}", e);
        },
    };
}
