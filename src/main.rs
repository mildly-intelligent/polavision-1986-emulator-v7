#![allow(unused)]
mod ansi_escape;
mod player;
mod map;
mod overlay;
mod inputs;

use std::{thread::sleep, time::Duration};

use ncurses::{clear, curs_set, endwin, getch, getmaxx, getmaxy, getmaxyx, initscr, refresh};

#[allow(unused_imports)]
use ansi_escape::{Color::*, Style::*, Other::*};
use player::{Player, MoveDir};
use map::Map;

use crate::overlay::*;

// use crate::image::{TEXT, draw_image};

const SCALE: (u16,u16) = (5,3);

fn main() {
    let win = initscr();
    let screen_w = getmaxx(win) as u16;
    let screen_h = getmaxy(win) as u16;
    println!("{}\x1b[?25l", Clear);

    let map:Map = Map::new("              -------           ",
                           "        ---00--     ------      ",
                           "---------                |      ",
                           "|                   1    +##### ",
                           "---------------          |    # ",
                           "              ---+--------    # ",
                           "  -------------  #         #### ",
                           "  |           |  #  ------ #    ",
                           "  |   2222    |  ###+    - #####",
                           "  |           |  #  ---+--     #",
                           "  --------+----  #     #       #",
                           "          ########     #########",);

    let mut player = Player {
        pos: (4,14),
        interact: None
    };
    
    loop {
        if inputs::handle_input(inputs::Context::Map, Some(&mut player), Some(&map), Some(win)) { break; }

        print!("{}{}", Clear, Home);
        player.get_interact(&map);
        map.draw(win, &mut player, SCALE);
        player.draw(win, &map, SCALE);
        println!();
    }
    dbg!(player);
}
