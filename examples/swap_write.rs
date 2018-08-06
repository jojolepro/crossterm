extern crate crossterm;

use crossterm::style::Color;
use crossterm::Crossterm;
use crossterm::terminal::ClearType;

fn main() {
    let mut crossterm = Crossterm::new();
    let (_, height) = crossterm.terminal().terminal_size();
        
    // [0,10]
    for i in 1..11 {
        crossterm.cursor().goto(0,height);
        crossterm.terminal().clear(ClearType::CurrentLine);
        crossterm.terminal().write(format!("{}\n\r>", i));
    }
}
