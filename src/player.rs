#[allow(unused_imports)]
use crate::ansi_escape::{Color::*, Style::*, Other::*};

#[allow(dead_code)]
pub enum MoveDir {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player<'a> {
    pub map: &'a crate::Map,
    pub pos: (u16, u16),
}
impl<'a> Player<'a> {
    pub fn mov(&mut self, dir: MoveDir) {
        let pos_old = self.pos.clone();
        match dir {
            MoveDir::Up => { self.pos.0 -= 1; },
            MoveDir::Down => { self.pos.0 += 1; },
            MoveDir::Left => { self.pos.1 -= 1; },
            MoveDir::Right => { self.pos.1 += 1; },
        }
        if self.pos.0 as usize >= self.map.0.len(){
            self.pos = pos_old;
            return;
        } else if self.pos.1 as usize >= self.map.0[self.pos.0 as usize].len(){
            self.pos = pos_old;
            return;
        }
        let c = self.map.0[self.pos.0 as usize].as_bytes()[self.pos.1 as usize];
        if ![b'.',b'+',b'#'].contains(&c) {
            self.pos = pos_old;
            return;
        }
    }

    pub fn draw(&self, (x, y, w, h): (u16,u16,u16,u16)) {
        for i in 0..h {
            print!("{}{}{}{}", Goto(self.pos.0*h+y+i, self.pos.1*w+x), Invert, "O".repeat(w as usize), NotInvert);
        }
    }
}