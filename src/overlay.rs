use std::{thread::sleep, time::Duration};

use ncurses::{getch, getmaxx, getmaxy};
use regex::Regex;

#[allow(unused_imports)]
use crate::ansi_escape::{Color::*, Style::*, Other::*};
use crate::inputs;

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

    pub fn print_new(w: *mut i8, text: &str) {
        let ansi_regex = Regex::new(r"\x1B\[(?:\d{1,4})?(?:;\d{1,4})*[A-HJm]").unwrap();

        let split: Vec<&str> = text.split('\n').collect();
        let width = split.iter()
            .map(|a| ansi_regex.replace_all(a, "").chars().count())
            .max()
            .unwrap() as u16;
        let height = split.len() as u16;

        let overlay = Self::new(w, width, height);

        overlay.draw_border();
        overlay.print(text);
    }

    pub fn print(&self, text: &str) {
        self.draw_border();
        let lines: Vec<char> = text.chars().collect();
        let lines = lines.chunks(self.w as usize);
        for (i, line) in lines.enumerate() {
            println!("{}{}", Goto(self.y+i as u16, self.x), line.iter().collect::<String>())
        }

        loop {
            if inputs::handle_input(inputs::Context::Overlay, None, None) { break; }
        }
    }
}

// pub fn draw_image(w: *mut i8, text: &str) {
//     let ansi_regex = Regex::new(r"\x1B\[(?:\d{1,4})?(?:;\d{1,4})*[A-HJm]").unwrap();

//     let screen_w = getmaxx(w) as usize;
//     let screen_h = getmaxy(w) as usize;

//     let x = (screen_w - width) as u16 / 2;
//     let y = (screen_h - height) as u16 / 2;
//     for (i, line) in text.split('\n').enumerate() {
//         print!("{}", Goto(y+i as u16, x));
//         for ch in line.split("") {
//             print!("{ch}");
//             sleep(Duration::from_millis(1));
//         }
//     }
//     sleep(Duration::from_secs(5));
// }