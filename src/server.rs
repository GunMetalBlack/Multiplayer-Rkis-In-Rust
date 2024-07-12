use std::{io::Read, net::{TcpListener, TcpStream}, thread};
use serde::de::value::Error;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Entity
{
    name: String,
    position: (u32, u32)
}
fn main() {
    println!("[Server Started]");
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    for stream in listener.incoming() {
        println!("[Client Connected]");
        let mut stream = stream.unwrap();
        thread::spawn(move || {
            handle_connection(&mut stream);
        });
    }
}

fn handle_connection(stream:&mut TcpStream)
{
    //Probably Big enough buffer Knock on wood
    let mut buffer = vec![0; 1024];
    let streamed_data = stream.read(&mut buffer).expect("Failed to unpack Entity on server");
    let received_data = &buffer[..streamed_data];
    let received_struct: Entity = serde_json::from_slice(received_data).expect("Failed to Serialize Player Struct We Done Fucked UP");
    println!("Received struct: {:?}", received_struct);
}
