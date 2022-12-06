use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn unique(s: &str) -> bool {
    let mut set: Vec<char> = vec![];
    for c in s.chars() {
        if set.contains(&c) {
            return false;
        }
        set.push(c);
    }
    return true;
}

fn find_unique(s: &String, len: usize) -> Option<usize> {
    for i in 0..=s.len() {
        if unique(&s[i..min(i + len, s.len())]) {
            return Some(i + len);
        }
    }
    None
}

pub fn run() -> Option<()> {
    let path = "./src/problem_6/input.txt";
    let input = File::open(path).unwrap();
    let lines = BufReader::new(input).lines();
    let mut sop: Option<usize> = None;
    let mut som: Option<usize> = None;
    for line in lines.flatten() {
        sop = find_unique(&line, 4);
        som = find_unique(&line, 14);
    }
    match sop {
        Some(i) => println!("First start-of-paket at {0}", i),
        None => println!("No first start-of-paket."),
    }
    match som {
        Some(i) => println!("First start-of-message at {0}", i),
        None => println!("No first start-of-message."),
    }
    None
}
