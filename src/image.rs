use std::{thread::sleep, time::Duration};

#[allow(unused_imports)]
use crate::ansi_escape::{Color::*, Style::*, Other::*};

pub fn draw_image(x: u16, y: u16, text: &str) {
    for (i, line) in text.split('\n').enumerate() {
        print!("{}", Goto(y+i as u16, x));
        for (j, ch) in line.split("").enumerate() {
            print!("{ch}");
            sleep(Duration::from_millis(1));
        }
    }
}