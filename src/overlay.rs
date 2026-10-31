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
    text: Option<String>,
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
            text: None,
        }
    }

    pub fn from(state: &State, text: &str) -> Self {
        let ansi_regex = Regex::new(r"\x1B\[(?:\d{1,4})?(?:;\d{1,4})*[A-HJm]").unwrap();
        let screen_w = getmaxx(state.win) as usize;
        let screen_h = getmaxy(state.win) as usize;

        let split: Vec<&str> = text.split('\n').collect();
        let width = split.iter()
            .map(|a| ansi_regex.replace_all(a, "").chars().count())
            .max()
            .unwrap() as u16;
        let height = split.len() as u16;

        Self {
            x: (screen_w as u16 - width) / 2,
            y: (screen_h as u16 - height) / 2,
            w: width,
            h: height,
            text: Some(String::from(text)),
        }
    }

    fn draw_border(&self) {
        let center = format!("{}     | {} |     {}", Column(self.x-7), " ".repeat(self.w as usize), Down(1));
        println!("{}     {}     ", Goto(self.y-2, self.x-7), " ".repeat(self.w as usize+4));
        println!("{}     {}     ", Column(self.x-7), "-".repeat(self.w as usize+4));
        println!("{}{}", center.repeat(self.h as usize), Up(1));
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

        overlay.print(state, text);
    }

    pub fn print(&self, state: &mut State, text: &str) {
        self.draw_border();
        // let lines: Vec<char> = text.chars().collect();
        // let lines = lines.chunks(self.w as usize);
        // let lines = wrap(text, self.w as usize);
        let lines = text.split('\n');
        for (i, line) in lines.enumerate() {
            println!("{}{}", Goto(self.y+i as u16, self.x), line)
        }
    }

    pub fn draw(&self, state: &mut State) { self.print(state, self.text.as_ref().unwrap().as_str()) }
}