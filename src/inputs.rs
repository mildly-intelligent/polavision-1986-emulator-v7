
use crate::{State, map::Map, player::{Player, MoveDir}};

use ncurses::{KEY_BACKSPACE, KEY_DOWN, KEY_LEFT, KEY_RIGHT, KEY_UP};

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Context {
    Map,
    Overlay,
    Pause,
    Title,
}

fn do_interact(state: &mut State, player: &mut Player, map: &Map) {
    if let Some(i) = player.interact {
        (map.get_interact(i).unwrap().action)(state, player, map)
    }
}

pub fn handle_input(key: i32, state: &mut State, player: Option<&mut Player>, map: Option<&Map>) -> bool {
    match state.ctx {
        Context::Map => match key {
            KEY_UP => player.unwrap().mov(MoveDir::Up, map.unwrap()),
            KEY_DOWN => player.unwrap().mov(MoveDir::Down, map.unwrap()),
            KEY_LEFT => player.unwrap().mov(MoveDir::Left, map.unwrap()),
            KEY_RIGHT => player.unwrap().mov(MoveDir::Right, map.unwrap()),
            10 => do_interact(state, player.unwrap(), &map.unwrap()),
            KEY_BACKSPACE => return true,
            _ => (),
        },
        Context::Overlay => match key {
            10 => {
                state.ctx = Context::Map;
                return true
            },
            _ => ()
        },
        Context::Pause => todo!(),
        Context::Title => match key {
            10 => return true,
            _ => (),
        },
    };
    false
}