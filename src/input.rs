use std::io;

pub fn get_input() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    line
}