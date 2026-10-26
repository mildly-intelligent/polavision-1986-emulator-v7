use ncurses::{addch, addstr, clear};

pub struct Map(pub Vec<String>);
impl Map {
    pub fn new(l0:&str,l1:&str,l2:&str,l3:&str,l4:&str,l5:&str,l6:&str,l7:&str,l8:&str,l9:&str,l10:&str,l11:&str) -> Self {
        Map(vec![
            l0.to_string(),
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
            l11.to_string()
        ])
    }
    pub fn disp(&self) -> Result<(), Box<dyn std::error::Error>> {
        clear();
        for line in &self.0 {
            addstr(line.as_str())?;
            addch('\n' as u32);
        }
        Ok(())
    }
}