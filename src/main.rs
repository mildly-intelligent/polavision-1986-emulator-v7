mod ansi_escape;
// mod map;
use ansi_escape::{Color::*, Style::*, Other::*};

fn main() {
    println!("{}This is a {}blue{} {}text{}{}tets", Clear, Blue,Default, Bold,NotBold, Goto(5,10));
}
