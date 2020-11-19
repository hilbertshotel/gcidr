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
        _ => 4,
    };

    if output != 1 {
        println!("{}", msg(output));
    }
}


fn stdin_match(pattern: &str) -> u8 {
    let mut output = 1;
    loop {

        if !ipv4::validate_pattern(pattern) {output=2; break;} 
        let stdin = io::stdin();
       
        let (pattern, range) = create_variables(pattern);

        for line in stdin.lock().lines() {
            let ip = match line {
                Ok(ip) => ip,
                Err(error) => panic!(error),
            };
            
            if !ipv4::validate_ip(&ip) {continue;}
            if ipv4::in_range(&ip, &pattern, &range) {println!("{}", ip);}
        }

        break;
    }
    output
}


fn file_match(pattern: &str, filename: &str) -> u8 {
    let mut output: u8 = 1;
    loop {
        
        if !Path::new(filename).exists() {output=3; break;}
        if !ipv4::validate_pattern(pattern) {output=2; break;}

        let file = OpenOptions::new()
            .read(true)
            .open(filename)
            .expect("error: open file");

        let (pattern, range) = create_variables(pattern);

        let file = BufReader::new(file);
        for line in file.lines() {
            let ip = line.expect("error: read line");
            if !ipv4::validate_ip(&ip) {continue;}
            if ipv4::in_range(&ip, &pattern, &range) {println!("{}", ip);}
        }

        break;
    }
    output
}


fn create_variables(pattern: &str) -> (Vec<&str>, Vec<u32>) {
    let pattern: Vec<&str> = pattern.split(|c| c == '.' || c == '/').collect();
    let cidr: u8 = pattern[4].parse().unwrap();
    let pattern = pattern[..=3].to_vec();
    let range: Vec<u32> = create_range(cidr);
    (pattern, range)
}


fn create_range(mut cidr: u8) -> Vec<u32> {
    let mut bin: [u8; 32] = [1; 32];
    let mut i: usize = 0;
    while cidr > 0 {
        bin[i] = 0;
        cidr -= 1;
        i += 1;
    }
    
    let bin = [&bin[..8], &bin[8..16], &bin[16..24], &bin[24..]];
    let mut range: Vec<u32> = Vec::new();
    for byte in &bin {
        let b: u8 = byte.iter().sum();
        let b = (2_u32.pow(b.into())) - 1;
        range.push(b);
    }
    range
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

