extern crate pancurses;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use pancurses::{curs_set, endwin, init_color, init_pair, initscr, noecho, start_color, Input, Window, COLOR_BLACK, COLOR_GREEN, COLOR_PAIR, COLOR_RED, COLOR_YELLOW};
use serde::de::value::Error;
use serde::{Serialize, Deserialize};
use std::io::{prelude::*, ErrorKind};
use std::net::TcpStream;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
struct Entity
{
    name: String,
    position: (u32, u32),
    color: i16
}
const  FOG_COLOR_CODE:image::Rgb<u8> =  image::Rgb([0, 0, 0]);
const  WALL_COLOR_CODE:image::Rgb<u8> =  image::Rgb([255, 0, 0]);

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:5000").expect("Failed to connect to the server");
    stream.set_nonblocking(true);
    let screen = initscr();
    //*Needed For Start Menu Dont Move!
    screen.keypad(true);
    curs_set(0);
    noecho();
    // Init The main player
    let mut player_entity = Entity{name:start_menu(&screen),position:(930, 558)};
    //Cleans the screen
    screen.refresh();
    screen.nodelay(true);
    engine(screen, &mut stream, &mut player_entity);
    endwin();
}


fn print_center_text(screen: &Window, message: String, text_y_offset: u32)
{
    let screen_width:u32  = screen.get_max_x().try_into().unwrap();
    let screen_height:u32 = screen.get_max_y().try_into().unwrap();
    let center_y = screen_height / 2;
    let center_x = screen_width / 2;
    let message_length:u32 = message.len() as u32;
    screen.mvprintw((center_y + text_y_offset).try_into().unwrap(), (center_x - message_length / 2).try_into().unwrap(), message);
}

fn menu_element_init(screen: &Window, element_name: String, is_selected: bool) -> String
{
    if is_selected
    {
        return element_name + "[X]";
    }
    else
    {
        return element_name + "[ ]";
    }
}


fn color_menu(screen: &Window) -> i16{
    print_center_text(screen, "[Pick Your Player Color]".to_string(), 0);
    let mut color_selection_index = 0;
    let mut element_array = [
    menu_element_init(screen, "Red".to_string(), false),
    menu_element_init(screen, "Green".to_string(), false),
    menu_element_init(screen, "Yellow".to_string(), false),
    menu_element_init(screen, "Blue".to_string(), false),
    menu_element_init(screen, "Purple".to_string(), false),
    menu_element_init(screen, "Cyan".to_string(), false),
    menu_element_init(screen, "White".to_string(), false)];
    loop {
        match screen.getch() {
            Some(Input::Character(c)) => {
                    if c == '\n' {break;}
                    if c == 'w'{},
                    if c == 's'{}
                 },
            _ => ()
        }
    }
    screen.refresh();
    return color_selection_index;
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
    screen.mvprintw((center_y + 2).try_into().unwrap(), (center_x - "Press Enter To Confirm -> Limit is 10 characters".len() as u32 / 2).try_into().unwrap(), "Press Enter To Confirm -> Limit is 10 characters");
    loop {
        match screen.getch() {
            Some(Input::Character(c)) => {
                    if c == '\n' {break;}
                    if user_name.len() as u32 >= 10 {break;}
                    user_name.push(c);
                    screen.mvprintw((center_y + 1).try_into().unwrap(), (center_x - user_name.len() as u32 / 2).try_into().unwrap(), &user_name);
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
    start_color();
    init_pair(1, COLOR_YELLOW, COLOR_BLACK);
    init_pair(2, COLOR_GREEN, COLOR_BLACK);
    // Loads the map through a png
    let mut real_map: ImageBuffer<image::Rgb<u8>, Vec<u8>> = load_map("test.png");
    //Loads the fog of war image:
    let mut shown_map: ImageBuffer<image::Rgb<u8>, Vec<u8>> = load_map("fog.png");
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
                    player_move(0, -1, player_entity, &mut shown_map, &real_map);
                    //player_entity.position.1 -= 1;
                    serialized_player_struct = serde_json::to_vec(&player_entity).expect("Failed to Serialize Player Struct We Done Fucked UP");
                    stream.write_all(&serialized_player_struct);
                } else if c == 'a' {
                    player_move(-1, 0, player_entity, &mut shown_map, &real_map);
                   // player_entity.position.0 -= 1;
                    serialized_player_struct = serde_json::to_vec(&player_entity).expect("Failed to Serialize Player Struct We Done Fucked UP");
                    stream.write_all(&serialized_player_struct);
                } else if c == 's' {
                    player_move(0, 1, player_entity, &mut shown_map, &real_map);
                    //player_entity.position.1 += 1;
                    serialized_player_struct = serde_json::to_vec(&player_entity).expect("Failed to Serialize Player Struct We Done Fucked UP");
                    stream.write_all(&serialized_player_struct);
                } else if c == 'd' {
                    player_move(1, 0, player_entity, &mut shown_map, &real_map);
                    //player_entity.position.0 += 1;
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
                &mut shown_map,
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
                    WALL_COLOR_CODE => {
                        screen.attron(COLOR_PAIR(1));
                        screen.mvaddstr(y as i32, x as i32,"█");
                        screen.attroff(COLOR_PAIR(1));
                    }
                    FOG_COLOR_CODE =>
                    {
                        screen.mvaddstr(y as i32, x as i32," ");
                    }
                    _ => {
                        screen.attron(COLOR_PAIR(2));
                        screen.mvaddch(y as i32, x as i32, '.');
                        screen.attroff(COLOR_PAIR(2));
                    }
                }
            }
        }
        for (client_id, client_entity) in client_player_map.iter()
        {
            let mut other_player_screen_space_x = client_entity.position.0 as i32 - (player_entity.position.0 as i32 - (screen_width as i32 / 2));
            let mut other_player_screen_space_y = client_entity.position.1 as i32 - (player_entity.position.1 as i32 - (screen_height as i32 / 2));
            init_pair(6, client_entity.color,COLOR_BLACK);
            if(other_player_screen_space_x <= screen_width as i32 && other_player_screen_space_y  <= screen_height as i32)
            { screen.attron(COLOR_PAIR(6)); screen.mvaddch(other_player_screen_space_y ,other_player_screen_space_x ,'P'); screen.attroff(COLOR_PAIR(6));}
        }
        screen.mvaddch((screen_height / 2) as i32, (screen_width / 2) as i32, 'P');
    }
}

fn player_move(x: i32, y:i32, player: &mut Entity, map: &mut ImageBuffer<image::Rgb<u8>, Vec<u8>>, real_map: &ImageBuffer<image::Rgb<u8>, Vec<u8>>){
    let temp_x = (player.position.0 as i32 + x) as u32;
    let temp_y = (player.position.1 as i32 + y) as u32;
    match real_map.get_pixel(temp_x, temp_y) {
       &WALL_COLOR_CODE => {
           return;
        }
        _ => {
            player.position.0 = temp_x;
            player.position.1 = temp_y;
            // Define the region to copy from the source image
            let src_x = temp_x - 5;
            let src_y = temp_y - 5;
            let width = 25;
            let height = 25;
            // Copy the region from the source image to the destination image
            for y_ in 0..height {
                for x_ in 0..width {
                    let pixel = real_map.get_pixel(src_x + x_, src_y + y_).clone();
                    map.put_pixel(src_x + x_, src_y + y_, pixel);
                }
            }
            //map.save("modified_destination_image.png").unwrap();
        }
    }

}