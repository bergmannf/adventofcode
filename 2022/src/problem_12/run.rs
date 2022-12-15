use std::cmp::min;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone)]
struct Graph {
    start: Node,
    end: Node,
    nodes: Vec<Vec<Node>>,
}

impl Graph {
    fn neighbors(&self, n: Node) -> Vec<&Node> {
        let mut nodes = vec![];
        let x_idx: usize = n.x.into();
        let y_idx: usize = n.y.into();
        let right_index: usize = min((n.x + 1).into(), self.nodes.len() - 1);
        let left_index: usize = if n.x == 0 { 0 } else { (n.x - 1).into() };
        let top_index: usize = min((n.y + 1).into(), self.nodes[x_idx].len() - 1);
        let bottom_index: usize = if n.y == 0 { 0 } else { (n.y - 1).into() };
        // println!(
        //     "X: {:?}, Y: {:?}, T: {:?}, B: {:?}, L: {:?}, R: {:?}",
        //     x_idx, y_idx, top_index, bottom_index, left_index, right_index
        // );
        if y_idx != top_index {
            nodes.push(&self.nodes[x_idx][top_index]);
        }
        if y_idx != bottom_index {
            nodes.push(&self.nodes[x_idx][bottom_index]);
        }
        if x_idx != left_index {
            nodes.push(&self.nodes[left_index][y_idx]);
        }
        if x_idx != right_index {
            nodes.push(&self.nodes[right_index][y_idx]);
        }
        nodes
    }

    fn reachable(&self, n: Node) -> Vec<&Node> {
        self.neighbors(n)
            .into_iter()
            .filter(|nb| nb.height <= n.height + 1)
            .collect()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
struct Node {
    x: u8,
    y: u8,
    height: u8,
}

fn grid_to_graph(grid: Vec<Vec<char>>) -> Graph {
    let mut nodes = vec![];
    let mut start: Node = Node {
        x: 0,
        y: 0,
        height: 0,
    };
    let mut end: Node = Node {
        x: 0,
        y: 0,
        height: 0,
    };
    for i in 0..grid.len() {
        let mut row = vec![];
        for j in 0..grid[i].len() {
            let c = grid[i][j];
            let node = match c {
                'S' => {
                    start = Node {
                        x: i as u8,
                        y: j as u8,
                        height: 0,
                    };
                    start
                }
                'E' => {
                    end = Node {
                        x: i as u8,
                        y: j as u8,
                        height: 25,
                    };
                    end
                }
                _ => {
                    let height = c as u8 - 97;
                    Node {
                        x: i as u8,
                        y: j as u8,
                        height,
                    }
                }
            };
            row.push(node);
        }
        nodes.push(row);
    }
    Graph { start, end, nodes }
}

fn parse_input(lines: Vec<String>) -> Graph {
    let mut grid = vec![];
    for line in lines {
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }
        grid.push(row);
    }
    grid_to_graph(grid)
}

fn path_to(maze: &Graph, start: &Node, end: &Node) -> usize {
    let mut queue = VecDeque::new();
    let mut from: HashMap<&Node, Option<&Node>> = HashMap::new();
    let mut current: Option<&Node> = None;
    from.insert(start, None);
    queue.push_back(start);
    while !queue.is_empty() {
        current = queue.pop_front();
        let c = current.unwrap();
        if c == end {
            break;
        }
        for nb in maze.reachable(*c) {
            if !from.contains_key(nb) {
                queue.push_back(nb);
                from.insert(nb, Some(c));
            }
        }
    }
    let mut path = vec![];
    if let Some(mut c) = current {
        while c != start {
            path.push(c);
            c = from[c].unwrap();
        }
    }
    path.len()
}

fn problem_a(maze: Graph) {
    println!("Finding path from {:?} to {:?}", &maze.start, &maze.end);
    let cost = path_to(&maze, &maze.start, &maze.end);
    println!(
        "Getting from {:?} to {:?} costs: {:?}",
        &maze.start, &maze.end, cost
    );
}

fn problem_b(maze: Graph) {}

pub fn run() {
    let path = "./src/problem_12/input.txt";
    let f = File::open(path).unwrap();
    let lines = BufReader::new(f).lines().flatten().collect();
    let maze = parse_input(lines);
    let m2 = maze.clone();
    problem_a(maze);
    problem_b(m2);
}

#[test]
fn test_parse_graph() {
    let grid = "abcd
efgh
SklE"
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let graph = parse_input(grid);
    assert_eq!(
        graph.start,
        Node {
            height: 0,
            x: 2,
            y: 0
        }
    );
    assert_eq!(
        graph.end,
        Node {
            height: 25,
            x: 2,
            y: 3
        }
    );
}

#[test]
fn test_neighbors() {
    let grid = "abcd
efgh
SijE"
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let graph = parse_input(grid);
    assert_eq!(
        graph.neighbors(graph.start),
        vec![
            &Node {
                height: 4,
                x: 1,
                y: 0
            },
            &Node {
                height: 8,
                x: 2,
                y: 1
            }
        ]
    );
    assert_eq!(
        graph.neighbors(graph.end),
        vec![
            &Node {
                height: 7,
                x: 1,
                y: 3
            },
            &Node {
                height: 9,
                x: 2,
                y: 2
            }
        ]
    )
}

#[test]
fn test_reachable() {
    let grid = "Sabd
bccc
dfgE"
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let graph = parse_input(grid);
    assert_eq! {
        graph.reachable(graph.start),
        vec![
            &Node {
                height: 0,
                x: 0,
                y: 1
            },
            &Node {
                height: 1,
                x: 1,
                y: 0
            },
        ]
    }
}

#[test]
fn test_cost() {
    let grid = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let graph = parse_input(grid);
    let cost = path_to(&graph, &graph.start, &graph.end);
    assert_eq!(cost, 31)
}
