use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let mut args = env::args();
    if args.len() >= 2 {
        args.next().unwrap();
        for filename in args {
            match File::open(filename) {
                Ok(file) => {
                    let mut buf_reader = BufReader::new(file);
                    do_cat(&mut buf_reader);
                }
                Err(e) => println!("{}", e),
            }
        }
    } else {
        let stdin = io::stdin();
        do_cat(&mut stdin.lock());
    }
}

fn do_cat(br: &mut dyn BufRead) -> () {
    let mut buffer = String::new();
    loop {
        match br.read_to_string(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                println!("{}", buffer);
                buffer.clear();
            }
            Err(e) => {
                println!("{}", e);
                break;
            }
        }
    }
}
