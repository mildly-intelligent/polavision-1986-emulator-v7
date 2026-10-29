use crate::{player::Player, map::Map};

pub struct State {
    pub win: *mut i8,
    pub player: Player,
    pub map: Map,
}

impl State {
    pub fn get_interact(&mut self) -> Option<&crate::map::Interact> {
        self.map.get_interact(self.player.interact.unwrap())
    }
}