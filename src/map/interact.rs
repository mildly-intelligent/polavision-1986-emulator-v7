use crate::utils::*;
use crate::pad;

use ncurses::{getmaxx, getmaxy};

use crate::{State, utils::ansi_escape::Other::*, map::Map, player::Player};

pub type InteractAction = dyn Fn(&mut State, &mut Player, &Map) -> ();

pub struct Interact {
    pub name: Option<String>,
    pub description: String,
    pub action: Box<InteractAction>,
}
impl Default for Interact {
    fn default() -> Self {
        Self {
            name: None,
            description: String::from("Default action"),
            action: Box::new(|w,p,m| ())
        }
    }
}
impl Interact {
    pub fn draw(&self, state: &mut State) {
        let (scrw, scrh) = (getmaxx(state.win) as u16, getmaxy(state.win) as u16);

        let x = scrw / 8;
        let state_y = (scrh - (crate::MAP_ROWS as u16*crate::SCALE.1))/3 + crate::SCALE.1*12;
        let y = (state_y + scrh) / 2;
        let w = scrw * 1/8;
        println!("{}{}", Goto(state_y+1, x), "~".repeat(w as usize));
        if let Some(name) = &self.name {
            println!("{}{}", Goto(y-1, x), pad!(name, w as usize))
        }
        println!("{}{}", Goto(y+1, x), pad!(&self.description, w as usize));
        println!("{}{}", Goto(scrh as u16 - 1, x), "~".repeat(w as usize));
    }
}