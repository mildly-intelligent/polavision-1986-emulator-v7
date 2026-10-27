#![allow(unused)]
mod ansi_escape;
mod player;
mod map;
// mod image;

use std::{thread::sleep, time::Duration};

use ncurses::{clear, curs_set, endwin, getch, initscr, refresh};

#[allow(unused_imports)]
use ansi_escape::{Color::*, Style::*, Other::*};
use player::{Player, MoveDir};
use map::Map;

// use crate::image::{TEXT, draw_image};

fn main() {
    initscr();
    println!("{}\x1b[?25l", Clear);

    let map:Map = Map::new("              -------           ",
                           "        -------.....------      ",
                           "---------................|      ",
                           "|........................|      ",
                           "---------................+###   ",
                           "        ---------+--------  #   ",
                           "                 #  ------ ##   ",
                           "  -------------  ###+....| #    ",
                           "  |...........|  #  ---+-- #####",
                           "  |...........+###     #       #",
                           "  -------------        #       #",
                           "                       #########",);
    let mut player = Player {
        map: &map,
        pos: (4,14),
    };

    loop {
        match getch() {
            65 => player.mov(MoveDir::Up),
            66 => player.mov(MoveDir::Down),
            68 => player.mov(MoveDir::Left),
            67 => player.mov(MoveDir::Right),
            27 => { endwin(); },
            _ => continue,
        }

        print!("{}{}", Clear, Home);
        for (i, line) in map.0.iter().enumerate() {
            print!("{}{line}\n", Goto(i as u16+1, 1));
        }
        println!("{}{}O{}", Goto(player.pos.0+1, player.pos.1+1), Reset, Reset);
        // draw_image(2, 2, TEXT);
    }
}
