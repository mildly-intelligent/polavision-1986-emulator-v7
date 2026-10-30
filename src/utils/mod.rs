use std::sync::LazyLock;
use regex::Regex;

pub mod ansi_escape;

pub fn repeat_char(s: &mut String, c1: char, c2: &str, n: u16) {
    *s = s.replace(c1, c2.repeat(n.into()).as_str());
}

pub const ANSI_REGEX: LazyLock<Regex> = 
    LazyLock::new(|| Regex::new(r"\x1B\[(?:\d{1,4})?(?:;\d{1,4})*[A-HJm]").unwrap());

#[doc(hidden)]
pub fn pad(s: &String, c: char, n: usize) -> String {
    let len = ANSI_REGEX.replace_all(s, "").len();
    let pad = c.to_string().repeat((n - len)/2);
    format!("{}{s}{}", pad, pad)
}

#[macro_export]
macro_rules! pad {
    ($s:expr, $c:literal, $n:expr) => {
        pad($s, $c, $n)
    };
    ($s:expr, $n:expr) => {
        pad($s, ' ', $n)
    };
}