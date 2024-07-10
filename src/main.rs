extern crate pancurses;

use image::{GenericImage, GenericImageView, ImageBuffer, RgbImage};
use pancurses::{endwin, initscr, noecho, Input, Window};

fn main() {
    let screen = initscr();
    //screen.printw("Sus");
    screen.refresh();
    screen.nodelay(true);
    screen.keypad(true);
    noecho();
    engine(screen);
    endwin();
}

fn load_map(filename: &str) -> RgbImage {
    let img = image::open(filename).unwrap();
    img.into_rgb8()
}

fn engine(screen: Window) {
    let mut map = load_map("test.png");
    let mut player_pos = (256, 256);
    loop {
        match screen.getch() {
            Some(Input::Character(c)) => {
                if c == 'w' {
                    player_pos.1 -= 1;
                } else if c == 'a' {
                    player_pos.0 -= 1
                } else if c == 's' {
                    player_pos.1 += 1
                } else if c == 'd' {
                    player_pos.0 += 1
                }
            }
            //Some(input) => {screen.addstr(&format!("{:?}", input)); },
            _ => (),
        }
        let screen_width = screen.get_max_x().try_into().unwrap();
        let screen_height = screen.get_max_y().try_into().unwrap();
            let subimg = image::imageops::crop(
                &mut map,
                player_pos.0 - (screen_width / 2),
                player_pos.1 - (screen_height / 2),
                screen_width,
                screen_height,
            );
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
