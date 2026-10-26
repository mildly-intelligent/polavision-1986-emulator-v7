#[allow(dead_code)]
pub enum MoveDir {
    Up,
    Down,
    Left,
    Right,
}

pub struct Player<'a> {
    pub map: &'a crate::Map,
    pub pos: (i32, i32),
}
impl<'a> Player<'a> {
    pub fn mov(&mut self, dir: MoveDir) {
        let pos_old = self.pos.clone();
        let c_old = self.map.0[self.pos.0 as usize].as_bytes()[self.pos.1 as usize];
        match dir {
            MoveDir::Up => { self.pos.0 -= 1; },
            MoveDir::Down => { self.pos.0 += 1; },
            MoveDir::Left => { self.pos.1 -= 1; },
            MoveDir::Right => { self.pos.1 += 1; },
        }
        dbg!(pos_old);
        dbg!(self.pos);
        let c = self.map.0[self.pos.0 as usize].as_bytes()[self.pos.1 as usize];
        if [b'-',b'|'].contains(&c) {
            self.pos = pos_old;
        }
        if c_old == b'#' && ![b'+',b'#'].contains(&c) {
            self.pos = pos_old;
        }
    }
    pub fn disp(&self) -> Result<(), Box<dyn std::error::Error>> {
        ncurses::mvaddstr(self.pos.0, self.pos.1, "O")?;
        Ok(())
    }
}