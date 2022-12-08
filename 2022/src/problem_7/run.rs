use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Tree {
    name: String,
    size: usize,
    is_dir: bool,
    children: Vec<Tree>,
}

impl Tree {
    fn new(name: &str) -> Tree {
        Self {
            name: name.to_string(),
            size: 0,
            is_dir: false,
            children: Vec::new(),
        }
    }

    fn find(&mut self, path: &[String]) -> &mut Self {
        let mut current = self;
        for part in path {
            current = current
                .children
                .iter_mut()
                .find(|x| x.name == *part)
                .unwrap();
        }
        current
    }

    fn set_size(&mut self) -> () {
        for c in self.children.iter_mut() {
            c.set_size()
        }
        if self.is_dir {
            self.size = self.children.iter_mut().map(|x| x.size).sum()
        }
    }

    fn all_children(&self) -> Vec<Self> {
        let mut children = Vec::new();
        if self.is_dir {
            for c in &self.children {
                children.push(c.clone());
                children.extend(c.all_children());
            }
        }
        children
    }
}

fn parse_lines(lines: Vec<String>) -> Tree {
    let mut tree = Tree::new("/");
    let mut current: Vec<String> = Vec::new();

    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts.len() == 0 {
            continue;
        }
        if parts[..2] == ["$", "cd"] {
            let directory = parts[2];
            match directory {
                "/" => continue,
                ".." => {
                    current.pop().unwrap();
                    continue;
                }
                x => {
                    current.push(x.to_owned());
                }
            }
        }
        if parts[0] == "dir" {
            let parent = tree.find(&current);
            parent.is_dir = true;
            if let Some(i) = parent.children.iter_mut().find(|x| x.name == parts[1]) {
                i.is_dir = true;
                continue;
            }
            let mut child = Tree::new(parts[1]);
            child.is_dir = true;
            parent.children.push(child);
        }
        if let Ok(i) = parts[0].parse::<usize>() {
            let mut file = Tree::new(parts[1]);
            file.size = i;
            tree.find(&current).children.push(file);
        }
    }
    tree.set_size();
    tree
}

pub fn run() -> Option<()> {
    let path = "./src/problem_7/input.txt";
    let input = File::open(path).unwrap();
    let lines = BufReader::new(input).lines().flatten().collect();
    let tree = parse_lines(lines);
    let problem_1: usize = tree
        .all_children()
        .iter()
        .filter(|t| t.is_dir && t.size <= 100_000)
        .map(|t| t.size)
        .sum();
    println!("Problem 1: {0}", problem_1);
    let available = 21_618_835;
    let update = 30_000_000;
    let required = update - available;
    let mut children = tree.all_children();
    children.sort_by(|a, b| a.size.cmp(&b.size));
    let smallest = children.iter().find(|t| t.size >= required).unwrap();
    println!("Smallest directory to delete: {:?}", smallest);
    None
}
