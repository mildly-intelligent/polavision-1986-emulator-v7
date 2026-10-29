#![allow(unused)]
mod ansi_escape;
mod player;
mod map;
mod overlay;

use std::{thread::sleep, time::Duration};

use ncurses::{clear, curs_set, endwin, getch, getmaxx, getmaxy, getmaxyx, initscr, refresh};

#[allow(unused_imports)]
use ansi_escape::{Color::*, Style::*, Other::*};
use player::{Player, MoveDir};
use map::Map;

use crate::overlay::*;

const TEXT: &str = "THIS IS A REALLY COOL TEST EXAMPLE ALSO FUCK YOU DERMOTT";

// use crate::image::{TEXT, draw_image};

const SCALE: (u16,u16) = (5,3);

fn do_interact(w: *mut i8, player: &Player) {
    if let Some(i) = player.interact { match i {
        b'0' => Overlay::print_new(w, TEXT),
        b'1' => Overlay::print_new(w, TEXT),
        _ => ()
    }}
}

fn main() {
    let w = initscr();
    let screen_w = getmaxx(w) as u16;
    let screen_h = getmaxy(w) as u16;
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
        map: &map,
        pos: (4,14),
        interact: None
    };

    let world_scale = (
        (screen_w - (map.0[0].len() as u16*SCALE.0))/2,
        (screen_h - (map.0.len() as u16*SCALE.1))/2,
        SCALE.0,
        SCALE.1
    );
    
    loop {
        // break;

        match getch() {
            65 => player.mov(MoveDir::Up),
            66 => player.mov(MoveDir::Down),
            68 => player.mov(MoveDir::Left),
            67 => player.mov(MoveDir::Right),
            10 => do_interact(w, &player),
            _ => (),
        }

        print!("{}{}", Clear, Home);
        player.get_interact();
        map.draw(&mut player, world_scale);
        player.draw(world_scale);
        println!();
    }
    dbg!(player);
}
