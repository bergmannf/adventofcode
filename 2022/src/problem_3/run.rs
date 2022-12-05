use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone)]
struct Backpack {
    total_content: String,
    left_content: String,
    right_content: String,
}

impl Display for Backpack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Total: {0}\nLeft:  {1}\nRight: {2}",
            self.total_content, self.left_content, self.right_content
        )
    }
}

impl Backpack {
    fn find_equal(&self) -> Option<char> {
        for cl in self.left_content.chars() {
            for cr in self.right_content.chars() {
                if cl == cr {
                    return Some(cl);
                }
            }
        }
        None
    }
}

fn read_input(path: String) -> Option<Vec<Backpack>> {
    let mut backpacks = Vec::<Backpack>::new();
    let f = File::open(path).ok()?;
    let lines = io::BufReader::new(f).lines();
    for line in lines.flatten() {
        let length = line.len();
        if length % 2 == 0 {
            let (left, right) = line.split_at(length / 2);
            backpacks.push(Backpack {
                total_content: line.clone(),
                left_content: left.to_string(),
                right_content: right.to_string(),
            })
        }
    }
    Some(backpacks)
}

pub fn create_priorities() -> HashMap<char, i32> {
    let mut priorities = HashMap::new();
    let mut value = 1;
    for c in 'a'..='z' {
        priorities.insert(c, value);
        value += 1;
    }
    for c in 'A'..='Z' {
        priorities.insert(c, value);
        value += 1;
    }
    priorities
}

fn find_group(teams: &[Backpack]) -> Option<char> {
    let bp1 = &teams[0].total_content;
    let bp2 = &teams[1].total_content;
    let bp3 = &teams[2].total_content;
    for c1 in bp1.chars() {
        for c2 in bp2.chars() {
            for c3 in bp3.chars() {
                if (c1 == c2) && (c1 == c3) {
                    println!("Found groups: {0}", c1);
                    return Some(c1);
                }
            }
        }
    }
    None
}

pub fn run() -> Option<()> {
    let priorities = create_priorities();
    let backpacks = read_input("./src/problem_3/input.txt".to_string())?;
    let teams = backpacks.clone();
    let mut sum = 0;
    for bp in backpacks {
        let equal = bp.find_equal()?;
        let value = priorities[&equal];
        sum += value;
    }
    let mut sum_groups = 0;
    for team in teams.chunks(3) {
        let group = find_group(team)?;
        let value = priorities[&group];
        sum_groups += value;
    }
    println!("Total value of duplicates {0}", sum);
    println!("Total value of groups {0}", sum_groups);
    None
}
