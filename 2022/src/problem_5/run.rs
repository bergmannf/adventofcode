use std::{fmt::Display, fs};

struct Stacks {
    values: Vec<Vec<char>>,
}

impl Display for Stacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, v) in self.values.iter().enumerate() {
            write!(f, "{0}: ", i);
            for c in v {
                write!(f, "{0}", c);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

impl Stacks {
    fn do_move(&mut self, m: Move) -> () {
        let from = m.from - 1;
        let to = m.to - 1;
        println!("Moving {0} crates: ", m.value);
        for i in 1..=m.value {
            println!("Move {0}", i);
            let maybe = self.values[from as usize].pop();
            match maybe {
                Some(v) => self.values[to as usize].push(v),
                None => (),
            }
        }
    }

    fn do_move_multi(&mut self, m: Move) -> () {
        let from = m.from - 1;
        let to = m.to - 1;
        let mut temp_vec: Vec<char> = vec![];
        for i in 1..=m.value {
            println!("Move {0}", i);
            let maybe = self.values[from as usize].pop();
            match maybe {
                Some(v) => temp_vec.push(v),
                None => (),
            }
        }
        for _ in 1..=temp_vec.len() {
            let v = temp_vec.pop().expect("No more elements.");
            self.values[to as usize].push(v);
        }
    }
}

struct Move {
    from: i32,
    to: i32,
    value: i32,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0} from {1} to {2}", self.value, self.from, self.to)
    }
}

fn parse_moves(input: &String) -> Vec<Move> {
    let mut moves: Vec<Move> = Vec::<Move>::new();
    for line in input.lines() {
        let mut parts = line.split(" ");
        let value = parts
            .nth(1)
            .expect("Second field not found")
            .parse::<i32>()
            .unwrap();
        let from = parts
            .nth(1)
            .expect("Fourth field not found")
            .parse::<i32>()
            .unwrap();
        let to = parts
            .nth(1)
            .expect("Sixth field not found")
            .parse::<i32>()
            .unwrap();
        println!("Building move for {0} - {1} - {2}", from, to, value);
        moves.push(Move { from, to, value })
    }
    moves
}

fn parse_stacks(input: &String) -> Stacks {
    let lines = input.lines();
    let number_of_stacks = lines.last().expect("a");
    println!("Number of stacks line: {0}", number_of_stacks);
    Stacks {
        values: vec![
            vec!['H', 'T', 'Z', 'D'],
            vec!['Q', 'R', 'W', 'T', 'G', 'C', 'S'],
            vec!['P', 'B', 'F', 'Q', 'N', 'R', 'C', 'H'],
            vec!['L', 'C', 'N', 'F', 'H', 'Z'],
            vec!['G', 'L', 'F', 'Q', 'S'],
            vec!['V', 'P', 'W', 'Z', 'B', 'R', 'C', 'S'],
            vec!['Z', 'F', 'J'],
            vec!['D', 'L', 'V', 'Z', 'R', 'H', 'Q'],
            vec!['B', 'H', 'G', 'N', 'F', 'Z', 'L', 'D'],
        ],
    }
}

fn parse_input(path: String) -> Option<(Stacks, Vec<Move>)> {
    let content = fs::read_to_string(path).expect("File did not exist.");
    let parts = content.split("\n\n");
    let parts_vec: Vec<String> = parts.map(|s| s.to_string()).collect();
    if parts_vec.len() != 2 {
        panic!("Expected input to contain 2 sections");
    }
    let stacks = parse_stacks(&parts_vec[0]);
    let moves = parse_moves(&parts_vec[1]);
    Some((stacks, moves))
}

pub fn run() -> Option<()> {
    let (mut state, moves) = parse_input("./src/problem_5/input.txt".to_string())?;
    for m in moves {
        state.do_move_multi(m);
    }
    for (i, v) in state.values.iter().enumerate() {
        let last = v.last();
        match last {
            Some(v) => println!("{0} last: {1}", i, v),
            None => println!("{0} is empty", i),
        }
    }
    None
}
