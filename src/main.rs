mod ansi_escape;
mod player;

use core::time;
use std::thread;
fn sleep(millis: u64) {
    thread::sleep(time::Duration::from_millis(millis));
}

#[allow(unused_imports)]
use ansi_escape::{Color::*, Style::*, Other::*};
use player::{Player, MoveDir};

struct Map(Vec<String>);
impl Map {
    fn new(l0:&str,l1:&str,l2:&str,l3:&str,l4:&str,l5:&str,l6:&str,l7:&str,l8:&str,l9:&str,l10:&str,l11:&str) -> Self {
        Map(vec![
            l0.to_string(),
            l1.to_string(),
            l2.to_string(),
            l3.to_string(),
            l4.to_string(),
            l5.to_string(),
            l6.to_string(),
            l7.to_string(),
            l8.to_string(),
            l9.to_string(),
            l10.to_string(),
            l11.to_string()
        ])
    }
}
impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            write!(f, "{line}\n")?;
        }
        Ok(())
    }
}

fn main() {
    let map = Map::new("                                ",
                       "        ------------------      ",
                       "        |                |      ",
                       "        |                |      ",
                       "        |                |      ",
                       "        ---------+--------      ",
                       "                 #              ",
                       "  -------------  #              ",
                       "  |           |  #              ",
                       "  |           +###              ",
                       "  -------------                 ",
                       "                                ",);
    let mut player = Player {
        map: &map,
        pos: (5,14),
    };
    println!("{}{map}{player}{}", Clear, Goto(12, 32));
    player.mov(MoveDir::Right);
    println!("{}{map}{player}{}", Clear, Goto(12, 32));
    player.mov(MoveDir::Right);
    println!("{}{map}{player}{}", Clear, Goto(12, 32));
    player.mov(MoveDir::Up);
    println!("{}{map}{player}{}", Clear, Goto(12, 32));
    player.mov(MoveDir::Up);
    println!("{}{map}{player}{}", Clear, Goto(12, 32));
    player.mov(MoveDir::Up);
    println!("{}{map}{player}{}", Clear, Goto(12, 32));
    player.mov(MoveDir::Up);
    println!("{}{map}{player}{}", Clear, Goto(12, 32));
    player.mov(MoveDir::Left);
    println!("{}{map}{player}{}", Clear, Goto(12, 32));
}
