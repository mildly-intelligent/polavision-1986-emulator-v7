use std::fmt::Display;
#[allow(unused_imports)]
use crate::ansi_escape::{Color::*, Style::*, Other::*};
use crate::sleep;

#[allow(dead_code)]
pub enum MoveDir {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player<'a> {
    pub map: &'a crate::Map,
    pub pos: (i16, i16),
}
impl<'a> Player<'a> {
    pub fn mov(&mut self, dir: MoveDir) {
        let pos_old = self.pos.clone();
        let c_old = self.map.0[self.pos.0 as usize].as_bytes()[self.pos.1 as usize];
        match dir {
            MoveDir::Up => {
                sleep(400);
                self.pos.0 -= 1;
            },
            MoveDir::Down => {
                sleep(400);
                self.pos.0 += 1;
            },
            MoveDir::Left => {
                sleep(200);
                self.pos.1 -= 1;
            },
            MoveDir::Right => {
                sleep(200);
                self.pos.1 += 1;
            },
        }
        dbg!(pos_old);
        dbg!(self.pos);
        let c = self.map.0[self.pos.0 as usize - 1].as_bytes()[self.pos.1 as usize - 1];
        if [b'-',b'|'].contains(&c) {
            self.pos = pos_old;
        }
        if c_old == b'#' && ![b'+',b'#'].contains(&c) {
            self.pos = pos_old;
        }
    }
}
impl<'a> Display for Player<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}O", Goto(self.pos.0, self.pos.1))
    }
}