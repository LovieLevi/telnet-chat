use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::str;

fn handle_client(mut stream: TcpStream) {
    let _ = stream.write(b"Welcome to a simple telnet chat!\r\n");
    let _ = stream.write(b"Please enter a nick for you:");

    let mut authorised: bool = false;
    let mut name: &str = "Internal server error occured";
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(size) => {
                let message_u8: &[u8] = &buffer[0..size];
                let message: &str = match str::from_utf8(message_u8) {
                    Ok(text) => text,
                    Err(..) => break
                };

                if !authorised {
                    name = message;
                    authorised = true;
                    let _ = stream.write(b"\r\n\r\nHere we go! Now you can start chatting!\r\n");
                } else {
                    let _ = stream.write(format!("\r\n{}> ", name).as_bytes());
                }
                print!("{}", message);
                // let _ = stream.write(&message_u8);
            }
            Err(e) => {
                eprintln!("Error reading from socket: {}", e);
                break;
            }
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address");
    println!("Server listening on 127.0.0.1:8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }
}
