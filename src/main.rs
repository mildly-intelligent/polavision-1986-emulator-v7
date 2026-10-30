#![allow(unused)]
mod player;
mod map;
mod overlay;
mod inputs;
#[macro_use]
mod utils;

use std::{thread::sleep, time::Duration};

use ncurses::{clear, curs_set, endwin, getch, getmaxx, getmaxy, getmaxyx, initscr, refresh};

#[allow(unused_imports)]
use utils::ansi_escape::{Color::*, Style::*, Other::*};
use player::{Player, MoveDir};
use map::Map;

use crate::overlay::*;

const SCALE: (u16,u16) = (5,3);
const MAP_ROWS: usize = 12;

struct State {
    win: *mut i8,
    current_map: usize,
    ctx: inputs::Context,
}

fn main() {
    let mut state = State {
        win: initscr(),
        current_map: 0,
        ctx: inputs::Context::Map,
    };

    let screen_w = getmaxx(state.win) as u16;
    let screen_h = getmaxy(state.win) as u16;
    println!("{}\x1b[?25l", Clear);

    let maps: [Map; 2] = [
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
        ])
            .set_interact(b'0', Some("Window"), "Look out window", |state,p,m| Overlay::print_new(state, "text"))
            .set_interact(b'3', Some("Door"), "Go to room", |state,p,m| {state.current_map = 1; p.pos = (10,17)}),
        Map::new([
            "          ---------------       ",
            "          |             |       ",
            "          |             |       ",
            "          |             |       ",
            "          |             |       ",
            "          |             |       ",
            "          |             |       ",
            "          |             |       ",
            "          |             |       ",
            "          |             |       ",
            "          |             |       ",
            "          ------000------       ",
        ])
            .set_interact(b'0', Some("Door"), "Go back to room", |state,p,m| {state.current_map = 0; p.pos = (1,17);}),
    ];

    let mut player = Player {
        pos: (4,14),
        interact: None
    };
    
    let mut map = &maps[state.current_map];

    loop {
        if inputs::handle_input(&mut state, Some(&mut player), Some(&map)) { break; }
        
        map = &maps[state.current_map];

        print!("{}{}", Clear, Home);
        player.get_interact(&map);
        map.draw(&mut state, &mut player);
        player.draw(state.win, &map);

        println!();
    }
}
