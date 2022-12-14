use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

struct Assignment {
    first_start: i32,
    first_end: i32,
    second_start: i32,
    second_end: i32,
}

impl fmt::Display for Assignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{0}-{1},{2}-{3}",
            self.first_start, self.first_end, self.second_start, self.second_end
        )
    }
}

impl Assignment {
    fn is_contained(&self) -> bool {
        if self.first_start <= self.second_start && self.second_end <= self.first_end {
            return true;
        }
        if self.second_start <= self.first_start && self.first_end <= self.second_end {
            return true;
        }
        false
    }

    fn overlaps(&self) -> bool {
        if self.first_start <= self.second_start && self.first_end >= self.second_start {
            return true;
        }
        if self.second_start <= self.first_start && self.second_end >= self.first_start {
            return true;
        }
        false
    }
}

fn parse_assignment(line: String) -> Assignment {
    let first_start: i32;
    let first_end: i32;
    let second_start: i32;
    let second_end: i32;
    let matches: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
    let (first, second) = (matches[0].clone(), matches[1].clone());
    let firsts: Vec<i32> = first
        .split('-')
        .map(|s| s.to_string())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let seconds: Vec<i32> = second
        .split('-')
        .map(|s| s.to_string())
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    match firsts[..] {
        [a] => {
            first_start = a;
            first_end = a;
        }
        [a, b] => {
            first_start = a;
            first_end = b;
        }
        _ => {
            panic!("Unexpected input first pair")
        }
    }
    match seconds[..] {
        [a] => {
            second_start = a;
            second_end = a;
        }
        [a, b] => {
            second_start = a;
            second_end = b;
        }
        _ => {
            panic!("Unexpected input second pair")
        }
    }
    Assignment {
        first_start,
        first_end,
        second_start,
        second_end,
    }
}

fn read_input(path: String) -> Option<Vec<Assignment>> {
    let mut assignments = Vec::<Assignment>::new();
    let f = File::open(path).ok()?;
    let lines = BufReader::new(f).lines();
    for line in lines.flatten() {
        let a = parse_assignment(line);
        assignments.push(a);
    }
    Some(assignments)
}

pub fn run() {
    let assignments = read_input("./src/problem_4/input.txt".to_string()).unwrap();
    let contained = assignments
        .iter()
        .map(|a| i32::from(a.is_contained()))
        .sum::<i32>();
    println!("Contained assignments: {0}", contained);
    let overlaps = assignments
        .iter()
        .map(|a| i32::from(a.overlaps()))
        .sum::<i32>();
    println!("Overlapping assignments: {0}", overlaps);
}
