use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn parse_lines(lines: Vec<String>) -> HashMap<usize, isize> {
    // Vector with elements (cycle, X)
    let mut state = HashMap::new();
    let mut current_cycle = 1;
    state.insert(1, 1);
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        match parts[..] {
            [_noop] => {
                state.insert(current_cycle + 1, state[&current_cycle]);
                current_cycle += 1;
            }
            [_addx, amount] => {
                let x_add = amount.parse::<isize>().unwrap();
                state.insert(current_cycle + 1, state[&current_cycle]);
                current_cycle += 1;
                state.insert(current_cycle + 1, state[&current_cycle] + x_add);
                current_cycle += 1;
            }
            _ => {}
        }
    }
    state
}

fn signal_strength(cycle: usize, state: &HashMap<usize, isize>) -> isize {
    let x_value = state[&(cycle)];
    println!("X at cycle {0}: {1}", cycle, x_value);
    isize::try_from(cycle).unwrap() * x_value
}

fn problem_a(state: &HashMap<usize, isize>) {
    let cycles: Vec<usize> = vec![20, 60, 100, 140, 180, 220];
    let sum: Vec<isize> = cycles.iter().map(|c| signal_strength(*c, &state)).collect();
    println!("Signal strength at cycles: {:?}", sum);
    println!("Sum of cycles: {0}", sum.iter().sum::<isize>());
}

fn visible_at_cycle(cycle: &usize, value: &isize, columns: isize) -> bool {
    let ivalue = isize::try_from(*cycle - 1).unwrap() % columns;
    let sprite_positions: Vec<isize> = vec![ivalue - 1, ivalue, ivalue + 1];
    sprite_positions.contains(&(value % columns))
}

fn problem_b(state: &HashMap<usize, isize>) {
    let mut crt = String::new();
    let mut keys = state.keys().collect::<Vec<&usize>>();
    keys.sort();
    for (i, key) in keys.iter().enumerate() {
        println!("{2}: Checking {0} -> {1}", key, state[key], i);
        if visible_at_cycle(key, &state[key], 40) {
            crt.push('#');
        } else {
            crt.push('.');
        }
        if *key % 40 == 0 {
            crt.push('\n');
        }
    }
    println!("{0}", crt);
}

pub fn run() {
    let path = "./src/problem_10/input.txt";
    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines().flatten().collect();
    let state = parse_lines(lines);
    problem_a(&state);
    problem_b(&state);
}
