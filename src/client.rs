extern crate pancurses;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use pancurses::{endwin, initscr, noecho, Input, Window};
use serde::de::value::Error;
use serde::{Serialize, Deserialize};
use std::io::{prelude::*, ErrorKind};
use std::net::TcpStream;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Entity
{
    name: String,
    position: (u32, u32)
}

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:5000").expect("Failed to connect to the server");
    stream.set_nonblocking(true);
    let screen = initscr();
    //*Needed For Start Menu Dont Move!
    screen.keypad(true);
    noecho();
    // Init The main player
    let mut player_entity = Entity{name:start_menu(&screen),position:(256, 256)};
    //Cleans the screen
    screen.refresh();
    screen.nodelay(true);
    engine(screen, &mut stream, &mut player_entity);
    endwin();
}
//Forgive my warcrimes here I know it could be simplified yes I am very aware.
fn start_menu(screen: &Window) -> String {
    let mut user_name:String = "".to_string();
    let screen_width:u32  = screen.get_max_x().try_into().unwrap();
    let screen_height:u32 = screen.get_max_y().try_into().unwrap();
    let center_y = screen_height / 2;
    let center_x = screen_width / 2;
    let message = "User-Name:";
    let message_length:u32 = message.len() as u32;
    screen.mvprintw(center_y.try_into().unwrap(), (center_x - message_length / 2).try_into().unwrap(), message);
    screen.mvprintw((center_y + 3).try_into().unwrap(), (center_x - "Press Enter To Confirm -> Limit is 10 characters".len() as u32 / 2).try_into().unwrap(), "Press Enter To Confirm -> Limit is 10 characters");
    loop {
        match screen.getch() {
            Some(Input::Character(c)) => {
                    if c == '\n' {break;}
                    if user_name.len() as u32 >= 10 {break;}
                    user_name.push(c);
                    screen.mvprintw((center_y + 2).try_into().unwrap(), (center_x - user_name.len() as u32 / 2).try_into().unwrap(), &user_name);
                 },
            _ => ()
        }
    }
    screen.refresh();
    return user_name;
}

//Loads the image from disk by file name 
fn load_map(filename: &str) -> RgbImage {
    let img = image::open(filename).unwrap();
    img.into_rgb8()
}

fn engine(screen: Window, stream: &mut TcpStream, player_entity: &mut Entity) {
    // Loads the map through a png
    let mut map = load_map("test.png");
    //Holds the list of players on the network
    let mut client_player_map: HashMap<String,Entity> = HashMap::new();
    //Test
    let mut serialized_player_struct = serde_json::to_vec(&player_entity).expect("Failed to Serialize Player Struct We Done Fucked UP");
    stream.write_all(&serialized_player_struct);
    //stream.read(&mut buffer);
    //Main game Loop
    loop {
        match screen.getch() {
            Some(Input::Character(c)) => {
                if c == 'w' {
                    player_entity.position.1 -= 1;
                    serialized_player_struct = serde_json::to_vec(&player_entity).expect("Failed to Serialize Player Struct We Done Fucked UP");
                    stream.write_all(&serialized_player_struct);
                } else if c == 'a' {
                    player_entity.position.0 -= 1;
                    serialized_player_struct = serde_json::to_vec(&player_entity).expect("Failed to Serialize Player Struct We Done Fucked UP");
                    stream.write_all(&serialized_player_struct);
                } else if c == 's' {
                    player_entity.position.1 += 1;
                    serialized_player_struct = serde_json::to_vec(&player_entity).expect("Failed to Serialize Player Struct We Done Fucked UP");
                    stream.write_all(&serialized_player_struct);
                } else if c == 'd' {
                    player_entity.position.0 += 1;
                    serialized_player_struct = serde_json::to_vec(&player_entity).expect("Failed to Serialize Player Struct We Done Fucked UP");
                    stream.write_all(&serialized_player_struct);
                }
            }
            //Some(input) => {screen.addstr(&format!("{:?}", input)); },
            _ => (),
        }
        let screen_width = screen.get_max_x().try_into().unwrap();
        let screen_height = screen.get_max_y().try_into().unwrap();
            let subimg = image::imageops::crop(
                &mut map,
                player_entity.position.0 - (screen_width / 2),
                player_entity.position.1 - (screen_height / 2),
                screen_width,
                screen_height,
            );
        //Getting the other clients data
        let mut buffer = vec![0; 1024];
        match stream.read(&mut buffer){
            Ok(0) => break, // Connection is closed
            Ok(streamed_data) => {
                let received_data = &buffer[..streamed_data];
                let received_struct: Entity = serde_json::from_slice(received_data).expect("Failed to Serialize Player Struct We Done Fucked UP");
                client_player_map.insert(received_struct.name.clone(), received_struct);
            }, // Data is available
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => (), // No data available
            Err(e) => println!("Well shit: {}", e), // An actual error occurred
        }
        //get_cur_yx() below
        for x in 0..screen_width {
            for y in 0..screen_height {
                match subimg.get_pixel(x, y) {
                    image::Rgb([255, 0, 0]) => {
                        screen.mvaddch(y as i32, x as i32, '#');
                    }
                    _ => {
                        screen.mvaddch(y as i32, x as i32, '.');
                    }
                }
            }
        }

        screen.mvaddch((screen_height / 2) as i32, (screen_width / 2) as i32, 'P');
    }
}
