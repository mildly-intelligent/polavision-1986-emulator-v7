use crate::utils::*;

use std::{sync::LazyLock, thread::sleep, time::Duration};

use ncurses::{getch, getmaxx, getmaxy};
use regex::Regex;

#[allow(unused_imports)]
use crate::utils::ansi_escape::{Color::*, Style::*, Other::*};
use crate::{State, inputs};

pub struct Overlay {
    x: u16,
    y: u16,
    w: u16,
    h: u16,
}
impl Overlay {
    pub fn new(w: *mut i8, width: u16, height: u16) -> Self {
        let screen_w = getmaxx(w) as usize;
        let screen_h = getmaxy(w) as usize;

        Self {
            x: (screen_w as u16 - width) / 2,
            y: (screen_h as u16 - height) / 2,
            w: width,
            h: height,
        }
    }

    fn draw_border(&self) {
        println!("{}     {}     ", Goto(self.y-2, self.x-7), " ".repeat(self.w as usize+4));
        println!("{}     {}     ", Column(self.x-7), "-".repeat(self.w as usize+4));
        println!("{}", format!("{}     | {} |     ", Column(self.x-7), " ".repeat(self.w as usize)).repeat(self.h as usize));
        println!("{}     {}     ", Column(self.x-7), "-".repeat(self.w as usize+4));
        println!("{}     {}     ", Column(self.x-7), " ".repeat(self.w as usize+4));
    }

    pub fn print_new(state: &mut State, text: &str) {
        let split: Vec<&str> = text.split('\n').collect();
        let width = split.iter()
            .map(|a| ANSI_REGEX.replace_all(a, "").chars().count())
            .max()
            .unwrap() as u16;
        let height = split.len() as u16;

        let overlay = Self::new(state.win, width, height);

        overlay.draw_border();
        overlay.print(state, text);
    }

    pub fn print(&self, state: &mut State, text: &str) {
        self.draw_border();
        let lines: Vec<char> = text.chars().collect();
        let lines = lines.chunks(self.w as usize);
        for (i, line) in lines.enumerate() {
            println!("{}{}", Goto(self.y+i as u16, self.x), line.iter().collect::<String>())
        }

        state.ctx = inputs::Context::Overlay;
        loop {
            if inputs::handle_input(state, None, None) { state.ctx = inputs::Context::Map; break; }
        }
    }
}