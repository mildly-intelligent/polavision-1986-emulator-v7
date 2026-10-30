use std::collections::HashMap;

use ncurses::{addch, addstr, clear, getmaxx, getmaxy};

#[allow(unused_imports)]
use crate::ansi_escape::{Color::*, Style::*, Other::*};
use crate::{overlay::Overlay, player::Player};

fn repeat(s: &mut String, c1: char, c2: &str, n: u16) {
    *s = s.replace(c1, c2.repeat(n.into()).as_str());
}

pub type InteractAction = Box<dyn Fn(*mut i8, &Player, &Map)>;

pub struct Interact {
    pub description: String,
    pub action: InteractAction,
}
impl Default for Interact {
    fn default() -> Self {
        Self {
            description: String::from("Default action"),
            action: Box::new(|w,p,m| ())
        }
    }
}

// #[derive(Debug)]
pub struct Map{
    map: Vec<String>,
    interacts: HashMap<u8, Interact>,
}

impl Map {
    pub fn new(lines: [&str; 12]) -> Self {
        Map { 
            map: lines.map(|l| String::from(l)).to_vec(),
            interacts: HashMap::new(),
        }
    }

    pub fn rows(&self) -> usize { self.map.len() }
    pub fn cols(&self) -> usize { self.map.iter().map(|a| a.len()).max().unwrap() }

    pub fn at(&self, pos: (u16, u16)) -> Option<u8> {
        Some(*self.map.get(pos.0 as usize)?.as_bytes().get(pos.1 as usize)?)
    }

    pub fn draw(&self, win: *mut i8, player: &mut crate::player::Player, (w, h): (u16,u16)) {
        let x = (getmaxx(win) as u16 - (self.cols() as u16*w))/2;
        let y = (getmaxy(win) as u16 - (self.rows() as u16*h))/2;

        for (i, line) in self.map.iter().enumerate() {
            let mut line = line.clone();
            repeat(&mut line, ' ', " ", w);
            repeat(&mut line, '-', "-", w);
            repeat(&mut line, '|', "|", w);
            repeat(&mut line, '+', "+", w);
            repeat(&mut line, '#', "#", w);

            if let Some(c) = player.interact {
                repeat(&mut line, c as char, "*", w);
            }

            for n in '0'..='9' {
                repeat(&mut line, n, "?", w);
            }
            for n in 'A'..='F' {
                repeat(&mut line, n, "?", w);
            }

            // Gang what the fuck is this line 😔✌️
            line = line.replace("*".repeat(w.into()).as_str(), &format!("{}{}{}", Blink, "*".repeat(w.into()), NotBlink).to_string());

            for j in 0..h {
                print!("{}{line}", Goto(h*i as u16+y+j, x));
            }
        }
    }

    pub fn set_interact<F>(self, index: u8, description: &str, action: F) -> Self
    where F: Fn(*mut i8, &Player, &Map) -> () + 'static
    {
        let mut new = self;
        new.interacts.insert(
            index,
            Interact {
                description: String::from(description),
                action: Box::new(action),
            }
        );

        new
    }

    pub fn get_interact(&self, index: u8) -> Option<&Interact> {
        self.interacts.get(&index)
    }
}