use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Read};

fn main() {
    for filename in env::args() {
        if read_file(&filename).is_err() {
            println!("file not found");
        }
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("{}", line.unwrap());
    }
}

fn read_file(filename: &str) -> std::io::Result<()> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}
