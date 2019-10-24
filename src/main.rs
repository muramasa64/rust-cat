use std::env;
use std::io::{self, BufRead};

fn main() {
    for argument in env::args() {
        println!("argument: {}", argument)
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}
