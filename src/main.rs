#![allow(unused)]
mod player;
mod map;
mod overlay;
mod inputs;
#[macro_use]
mod utils;

use std::{io::Write, thread::sleep, time::Duration};

use ncurses::{cbreak, clear, curs_set, delscreen, endwin, filter, getch, getmaxx, getmaxy, getmaxyx, initscr, keyname, keypad, newscr, noecho, refresh};

#[allow(unused_imports)]
use utils::ansi_escape::{Color::*, Style::*, Other::*};
use player::{Player, MoveDir};
use map::Map;

use crate::overlay::*;

const SCALE: (u16,u16) = (5,3);
const MAP_ROWS: usize = 12;

fn wait_key() -> i32 {
    let mut key = -1;
    while key == -1 {
        key = getch();
    }
    key
}

struct State {
    win: *mut i8,
    current_map: usize,
    ctx: inputs::Context,
}

fn ellipsis(ticks: usize) {
    print!("{}{}", Up(1), Save);
    for t in 0..ticks {
        let t = t%4;
        println!("{}{}", Restore, ".".repeat(t) + &" ".repeat(3-t));
        sleep(Duration::from_millis(250));
    }
    println!("{}...", Restore);
}
fn startup_sequence() {
    let debug =   |msg: &str| format!("{}  {}{}  Debug{}{} {msg}", Column(1), Magenta,  Bold, NotBold, DefaultColor);
    let info =    |msg: &str| format!("{}  {}{}   Info{}{} {msg}", Column(1), Cyan,     Bold, NotBold, DefaultColor);
    let warning = |msg: &str| format!("{}  {}{}Warning{}{} {msg}", Column(1), Yellow,   Bold, NotBold, DefaultColor);
    let error =   |msg: &str| format!("{}  {}{}  Error{}{} {msg}", Column(1), Red,      Bold, NotBold, DefaultColor);

    // I have no idea if any of this is even remotely realistic
    println!("{}Thank you for using Recon-1986-emulator!", Column(1));
    println!("{}This is version 3 of the emulator, you can view the changelog at https://reconemulator.net/v3/changelog", Column(1));
    println!("{}Starting emulator with logging set to {}{}Debug{}{}", Column(1), Magenta, Bold, NotBold, DefaultColor);
    println!("{}", info("Initializing 4kB of virtual ram"));
    ellipsis(9);
    println!("{}", info("Loading program ROM"));
    ellipsis(8);
    println!("{}", warning("Map data may have been corrupted or lost when loading"));
    sleep(Duration::from_millis(250));
    println!("{}", debug("Failed to write over data in ROM at address $3000-$39FF (map data)"));
    sleep(Duration::from_millis(250));
    println!("{}", debug("Possible faulty initial RAM decay pattern"));
    sleep(Duration::from_millis(250));
    println!("{}", info("Initializing virtual display"));
    ellipsis(6);
    println!("{}", info("Initializing virtual controls panel"));
    ellipsis(3);
    println!("{}", info("Initializing audio pipe"));
    ellipsis(5);
    println!("{}", error("Compatible audio driver not found, skipping"));
    println!("{}", info("Finishing setup"));
    ellipsis(9);
    println!("{}", error("Failed to purge God, skipping"));
    print!("{}{}", debug("3000"), Save);
    for _ in 0..3000 {
        println!("{}3000{}", Restore, Save);
        sleep(Duration::from_micros(50))
    }
}

fn draw(state: &mut State, player: &mut Player, map: &Map) {
    print!("{}{}", Clear, Home);
    player.get_interact(map);
    map.draw(state, player);
    player.draw(state, map);
    println!();
}

fn main() {
    noecho();
    cbreak();

    let mut state = State {
        win: initscr(),
        current_map: 0,
        ctx: inputs::Context::Title,
    };
    keypad(state.win, true);

    let screen_w = getmaxx(state.win) as u16;
    let screen_h = getmaxy(state.win) as u16;

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
        pos: (3,17),
        interact: None
    };
    
    if !cfg!(debug_assertions) { startup_sequence() }

    println!("{}{}\x1b[?25l", Clear, Home);
    let title_card = overlay::Overlay::from(&state, "  __ __       _____    _____     _____      __   __   
 /_/\\__/\\   /\\_____\\  /\\ __/\\   ) ___ (    /_/\\ /\\_\\  
 ) ) ) ) ) ( (_____/  ) )__\\/  / /\\_/\\ \\   ) ) \\ ( (  
/_/ /_/_/   \\ \\__\\   / / /    / /_/ (_\\ \\ /_/   \\ \\_\\ 
\\ \\ \\ \\ \\   / /__/_  \\ \\ \\_   \\ \\ )_/ / / \\ \\ \\   / / 
 )_) ) \\ \\ ( (_____\\  ) )__/\\  \\ \\/_\\/ /   )_) \\ (_(  
 \\_\\/ \\_\\/  \\/_____/  \\/___\\/   )_____(    \\_\\/ \\/_/  

               Erimita Interactive LLC.
                      c. 1986");
    while !inputs::handle_input(getch(), &mut state, None, None) {
        title_card.draw(&mut state);
        println!("{}{}{}Insert 5¢ to begin{}", Down(3), Column((screen_w-18)/2), Blink, NotBlink);
    }
    let mut map = &maps[state.current_map];
    draw(&mut state, &mut player, &map);
    state.ctx = inputs::Context::Map;
    while !inputs::handle_input(getch(), &mut state, Some(&mut player), Some(&map)) {        
        map = &maps[state.current_map];
        draw(&mut state, &mut player, &map);
    }
}
