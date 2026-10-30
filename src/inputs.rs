use std::io::Write;

use ncurses::getch;

use crate::{State, map::Map, overlay::Overlay, player::{MoveDir, Player}};

#[derive(Clone, Copy)]
pub enum Context {
    Map,
    Overlay,
    Pause,
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
            65 => player.unwrap().mov(MoveDir::Up, map.unwrap()),
            66 => player.unwrap().mov(MoveDir::Down, map.unwrap()),
            68 => player.unwrap().mov(MoveDir::Left, map.unwrap()),
            67 => player.unwrap().mov(MoveDir::Right, map.unwrap()),
            10 => do_interact(state, &player.unwrap(), &map.unwrap()),
            20 => return true,
            _ => (),
        },
        Context::Overlay => match key {
            10 => return true,
            _ => ()
        },
        Context::Pause => todo!(),
    };
    false
}