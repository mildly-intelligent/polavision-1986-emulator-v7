#[allow(unused_imports)]
use crate::ansi_escape::{Color::*, Style::*, Other::*};

#[allow(dead_code)]
pub enum MoveDir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Player<'a> {
    pub map: &'a crate::Map,
    pub pos: (u16, u16),
    pub interact: Option<u8>,
}
impl<'a> Player<'a> {
    pub fn mov(&mut self, dir: MoveDir) {
        let pos_old = self.pos.clone();
        let c_old = self.map.at(self.pos).unwrap();
        self.pos = self.get(dir);
        if self.pos.0 as usize >= self.map.0.len() {
            self.pos = pos_old;
            return;
        } else if self.pos.1 as usize >= self.map.0[self.pos.0 as usize].len() {
            self.pos = pos_old;
            return;
        }
        let c = self.map.at(self.pos).unwrap();
        if [b'-',b'|',b'?',b'0',b'1',b'2',b'3',b'4',b'5',b'6',b'7',b'8',b'9',b'A',b'B',b'C',b'D',b'E',b'F'].contains(&c) {
            self.pos = pos_old;
            return;
        }
        if c_old == b'#' && c == b' ' {
            self.pos = pos_old;
            return;
        }
    }

    pub fn get(&self, dir: MoveDir) -> (u16,u16) {
        match dir {
            MoveDir::Up =>    (self.pos.0-1, self.pos.1  ),
            MoveDir::Down =>  (self.pos.0+1, self.pos.1  ),
            MoveDir::Left =>  (self.pos.0,   self.pos.1-1),
            MoveDir::Right => (self.pos.0,   self.pos.1+1),
        }
    }

    pub fn draw(&self, (x, y, w, h): (u16,u16,u16,u16)) {
        for i in 0..h {
            print!("{}{}", Goto(self.pos.0*h+y+i, self.pos.1*w+x), "█".repeat(w as usize));
        }
    }

    pub fn get_interact(&mut self) -> Option<(u16,u16)> {
        let pu = self.get(MoveDir::Up);
        let pd = self.get(MoveDir::Down);
        let pr = self.get(MoveDir::Right);
        let pl = self.get(MoveDir::Left);

        if let Some(cu) = self.map.at(pu) && cu.is_ascii_hexdigit() {
            self.interact = Some(cu);
            Some(pu)
        } else if let Some(cd) = self.map.at(pd) && cd.is_ascii_hexdigit() {
            self.interact = Some(cd);
            Some(pd)
        } else if let Some(cr) = self.map.at(pr) && cr.is_ascii_hexdigit() {
            self.interact = Some(cr);
            Some(pr)
        } else if let Some(cl) = self.map.at(pl) && cl.is_ascii_hexdigit() {
            self.interact = Some(cl);
            Some(pl)
        } else {
            self.interact = None;
            None
        }
    }
}