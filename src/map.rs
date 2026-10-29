use ncurses::{addch, addstr, clear, getmaxx, getmaxy};

#[allow(unused_imports)]
use crate::ansi_escape::{Color::*, Style::*, Other::*};

fn repeat(s: &mut String, c1: char, c2: &str, n: u16) {
    *s = s.replace(c1, c2.repeat(n.into()).as_str());
}

struct Interact {
    description: String,
    action: Box<dyn Fn() -> ()>,
}

// #[derive(Debug)]
pub struct Map{
    map: Vec<String>,
    interacts: Vec<Interact>,
}

impl Map {
    pub fn new(l0:&str,l1:&str,l2:&str,l3:&str,l4:&str,l5:&str,l6:&str,l7:&str,l8:&str,l9:&str,l10:&str,l11:&str) -> Self {
        Map { 
            map: vec![l0.to_string(),
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
                      l11.to_string()],
            interacts: Vec::new(),
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
        // if let Some(p) = player.get_interact() {
        //     for y in p.0*h+y..p.0*h+y+h {
        //         print!("{}{}{}{}", Goto(y, p.1*w+x), Blink, "?".repeat(w as usize), NotBlink);
        //     }
        // }
    }
}