use std::env;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Round {
    opponent_hand: String,
    player: String,
}

impl fmt::Display for Round {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Player: {0} - Opponent {1}",
            self.player, self.opponent_hand,
        )
    }
}

impl Round {
    fn play_hand(&self) -> i32 {
        // A, X: Rock
        // B, Y: Paper
        // C, Z: Scissor
        match (self.opponent_hand.as_str(), self.player.as_str()) {
            ("A", "X") => 3 + 1,
            ("A", "Y") => 6 + 2,
            ("A", "Z") => 0 + 3,
            ("B", "X") => 0 + 1,
            ("B", "Y") => 3 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "X") => 6 + 1,
            ("C", "Y") => 0 + 2,
            ("C", "Z") => 3 + 3,
            (_, _) => 0,
        }
    }

    fn play_result(&self) -> i32 {
        // X: Loose
        // Y: Draw
        // Z: Win
        match (self.opponent_hand.as_str(), self.player.as_str()) {
            ("A", "X") => 0 + 3,
            ("A", "Y") => 3 + 1,
            ("A", "Z") => 6 + 2,
            ("B", "X") => 0 + 1,
            ("B", "Y") => 3 + 2,
            ("B", "Z") => 6 + 3,
            ("C", "X") => 0 + 2,
            ("C", "Y") => 3 + 3,
            ("C", "Z") => 6 + 1,
            (_, _) => 0,
        }
    }
}

fn load_input(path: String) -> Option<Vec<Round>> {
    let mut plays = Vec::new();
    if let Ok(lines) = read_lines(path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines.flatten() {
            let mut split = line.split(' ');
            let opponent_hand = split.next()?;
            let player_hand = split.next()?;
            plays.push(Round {
                player: String::from(player_hand),
                opponent_hand: String::from(opponent_hand),
            })
        }
    }
    Some(plays)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn run() -> Option<()> {
    let root = match env::current_dir() {
        Ok(d) => d,
        Err(_) => panic!("Could not get current directory"),
    };
    let root_path = root.to_str().unwrap().to_string();
    let file_path = root_path + "/src/problem_2/input.txt";
    println!("Loading game data from {0}", file_path);
    let data = load_input(file_path);
    let mut sum_hand = 0;
    let mut sum_play = 0;
    if let Some(d) = data {
        for round in d {
            let r_hand = round.play_hand();
            sum_hand += r_hand;
            let r_play = round.play_result();
            sum_play += r_play;
        }
    }
    println!("Sum of rounds (play): {0}", sum_hand);
    println!("Sum of rounds (outcome): {0}", sum_play);
    None
}
