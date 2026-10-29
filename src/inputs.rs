use ncurses::getch;

use crate::{map::Map, overlay::Overlay, player::{MoveDir, Player}};

pub enum Context {
    Map,
    Overlay,
    Pause,
}

const TEXT: &str = "THIS IS A REALLY COOL TEST EXAMPLE ALSO FUCK YOU DERMOTT";

fn do_interact(w: *mut i8, player: &Player) {
    if let Some(i) = player.interact { match i {
        b'0' => Overlay::print_new(w, TEXT),
        b'1' => Overlay::print_new(w, TEXT),
        _ => ()
    }}
}

pub fn handle_input(ctx: Context, player: Option<&mut Player>, map: Option<&Map>, w: Option<*mut i8>) -> bool {
    let key = getch();
    match ctx {
        Context::Map => match key {
            65 => player.unwrap().mov(MoveDir::Up, map.unwrap()),
            66 => player.unwrap().mov(MoveDir::Down, map.unwrap()),
            68 => player.unwrap().mov(MoveDir::Left, map.unwrap()),
            67 => player.unwrap().mov(MoveDir::Right, map.unwrap()),
            10 => do_interact(w.unwrap(), &player.unwrap()),
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