use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    let file_handler = match File::open(filename) {
        Ok(file_handler) => file_handler,
        Err(_) => {
            println!("Error occured during opening file:: {}", filename);
            return;
        }
    };
    let (mut line_count, mut word_count, mut char_count): (usize, usize, usize) = (0, 0, 0);
    for line in BufReader::new(file_handler).lines() {
        let line_str = line.unwrap();
        line_count += 1;
        char_count += line_str.chars().count();
        word_count += line_str.split(' ').count();
    }

    println!("{} {} {} {}", line_count, word_count, char_count, filename);
}
