use std::env;
use std::path::Path;
use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader};

mod ipv4;


fn main() {
    let input: Vec<String> = env::args().collect();

    let output: u8 = match input.len() {
        1 => 0,
        2 => stdin_match(&input[1]),
        3 => file_match(&input[1], &input[2]),
        _ => 10,
    };

    if output != 1 {
        println!("{}", msg(output));
    }
}


fn stdin_match(pattern: &str) -> u8 {
    if !is_valid(pattern) {
        2
    } else {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            let ip = match line {
                Ok(ip) => ip,
                Err(error) => panic!(error),
            };
            if in_range(&ip, pattern) {
                println!("{}", ip);
            } 
        }
        1
    }
}


fn file_match(pattern: &str, filename: &str) -> u8 {
    if !Path::new(filename).exists() {
        3
    } else if !is_valid(pattern) {
        2
    } else {
        let file = OpenOptions::new()
            .read(true)
            .open(filename)
            .expect("error: open file");

        let file = BufReader::new(file);
        for line in file.lines() {
            let ip = line.expect("error: read line");
            if in_range(&ip, pattern) {
                println!("{}", ip);
            }
        }
        1
    }
}


fn is_valid(pattern: &str) -> bool {
    ipv4::validate(pattern)
}

fn in_range(_line: &str, _pattern: &str) -> bool {
    true
}


// ERROR HANDLING
fn msg(output: u8) -> String {
    match output {
        0 => "gcidr <pattern> <filename> or stdin | gcdir <pattern>".to_string(),
        2 => error("invalid pattern"),
        3 => error("file not found"),
        _ => error("too many arguments"),
    }
}


fn error(msg: &str) -> String {
    format!("error: {}", msg)
}

