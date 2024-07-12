use std::{io::{Read, Write}, net::{TcpListener, TcpStream}, thread};
use serde::de::value::Error;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
struct Entity
{
    name: String,
    position: (u32, u32)
}
fn main() {
    println!("[Server Started]");
    let listener = TcpListener::bind("127.0.0.1:5000").unwrap();
    // Create a mutex to protect the Stream data which will be passed to each thread
    let mut stream_list: Vec<TcpStream> = Vec::new();  
    let stream_list_mutex = Arc::new(Mutex::new(stream_list));
    for stream in listener.incoming() {
        println!("[Client Connected]");
        let mut stream = stream.unwrap();
        {
            //May god have mercy on the crimes commited
            let mut scoped_stream_list = stream_list_mutex.lock().unwrap();
            scoped_stream_list.push(stream.try_clone().unwrap()); // Clone the stream
        }
        let mut arc_clone_mutex = Arc::clone(&stream_list_mutex);
        thread::spawn(move || {
            handle_connection(&mut stream, arc_clone_mutex);
        });
    }
}

fn handle_connection(stream:&mut TcpStream, stream_list_mutex: Arc<Mutex<Vec<TcpStream>>>)
{
    //Probably Big enough buffer Knock on wood
    let mut buffer = vec![0; 1024];
    loop {
        let result = stream.read(&mut buffer);
        match result {
            Ok(streamed_data) => {
                let received_data = &buffer[..streamed_data];
                let received_struct: Entity = serde_json::from_slice(received_data).expect("Failed to Serialize Player Struct We Done Fucked UP");
                println!("Received struct: {:?}", received_struct);
                {
                    let mut teddy_list = stream_list_mutex.lock().unwrap();
                    for mut client_stream in teddy_list.iter() 
                    {
                        if(stream.peer_addr().unwrap() != client_stream.peer_addr().unwrap())
                        {
                            client_stream.write_all(received_data);
                        }
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to unpack Entity on server: {}", e);
                break; // exit the loop
            }
        }
    }
}
