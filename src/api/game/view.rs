use std::io::{self, Write};

pub fn get_input() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}

pub fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush().unwrap();
}
