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

struct State {
    win: *mut i8,
    current_map: usize,
    ctx: inputs::Context,
}

fn main() {
    let state = State {
        win: initscr(),
        current_map: 0,
        ctx: inputs::Context::Map,
    };

    let screen_w = getmaxx(state.win) as u16;
    let screen_h = getmaxy(state.win) as u16;
    println!("{}\x1b[?25l", Clear);

    let maps: [Map; 1] = [
        Map::new([
            "              ---3---           ",
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
            "          ########     #########",
        ]).set_interact(b'0', "description", |state,p,m| Overlay::print_new(state, "text")),
    ];

    let mut player = Player {
        pos: (4,14),
        interact: None
    };
    
    loop {
        let map = &maps[state.current_map];

        if inputs::handle_input(&state, Some(&mut player), Some(&map)) { break; }

        print!("{}{}", Clear, Home);
        player.get_interact(&map);
        map.draw(state.win, &mut player, SCALE);
        player.draw(state.win, &map, SCALE);
        println!();
    }
}
