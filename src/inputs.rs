use std::io::Write;

use ncurses::{KEY_BACKSPACE, KEY_DOWN, KEY_ENTER, KEY_LEFT, KEY_RIGHT, KEY_UP, getch, keyname};

use crate::{State, map::Map, overlay::Overlay, player::{MoveDir, Player}};

#[derive(Clone, Copy)]
pub enum Context {
    Map,
    Overlay,
    Pause,
    Title,
}

const TEXT: &str = "THIS IS A REALLY COOL TEST EXAMPLE ALSO FUCK YOU DERMOTT";

fn do_interact(state: &State, player: &Player, map: &Map) {
    if let Some(i) = player.interact {
        (map.get_interact(i).unwrap().action)(state, player, map)
    }

    // if let Some(i) = player.interact { match i {
    //     b'0' => Overlay::print_new(w, TEXT),
    //     b'1' => Overlay::print_new(w, TEXT),
    //     _ => ()
    // }}
}

pub fn handle_input(key: i32, state: &State, player: Option<&mut Player>, map: Option<&Map>) -> bool {
    match state.ctx {
        Context::Map => match key {
            KEY_UP => player.unwrap().mov(MoveDir::Up, map.unwrap()),
            KEY_DOWN => player.unwrap().mov(MoveDir::Down, map.unwrap()),
            KEY_LEFT => player.unwrap().mov(MoveDir::Left, map.unwrap()),
            KEY_RIGHT => player.unwrap().mov(MoveDir::Right, map.unwrap()),
            10 => do_interact(state, &player.unwrap(), &map.unwrap()),
            KEY_BACKSPACE => return true,
            _ => (),
        },
        Context::Overlay => match key {
            10 => return true,
            _ => (),
        },
        Context::Pause => todo!(),
        Context::Title => match key {
            10 => return true,
            _ => (),
        },
    };
    false
}