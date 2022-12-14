use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

enum Move {
    Up,
    Down,
    Left,
    Right,
    Invalid,
}

fn parse_line(line: &String) -> (Move, usize) {
    let parts = line.split_whitespace().collect::<Vec<_>>();
    match parts[..] {
        ["R", b] => (Move::Right, b.parse::<usize>().unwrap()),
        ["U", b] => (Move::Up, b.parse::<usize>().unwrap()),
        ["D", b] => (Move::Down, b.parse::<usize>().unwrap()),
        ["L", b] => (Move::Left, b.parse::<usize>().unwrap()),
        _ => (Move::Invalid, 0),
    }
}

fn move_tail(head: (isize, isize), tail: (isize, isize)) -> (isize, isize) {
    let mut new_x = tail.0;
    let mut new_y = tail.1;
    if !must_move(head, tail) {
        return tail;
    }
    if distance_x(head, tail) >= 2 && distance_y(head, tail) == 0 {
        if head.0 < tail.0 {
            new_x = tail.0 - 1
        } else {
            new_x = tail.0 + 1
        }
    } else if distance_y(head, tail) >= 2 && distance_x(head, tail) == 0 {
        if head.1 < tail.1 {
            new_y = tail.1 - 1
        } else {
            new_y = tail.1 + 1
        }
    } else {
        if head.0 < tail.0 {
            new_x = tail.0 - 1
        } else {
            new_x = tail.0 + 1
        }
        if head.1 < tail.1 {
            new_y = tail.1 - 1
        } else {
            new_y = tail.1 + 1
        }
    }
    (new_x, new_y)
}

fn distance_x(head: (isize, isize), tail: (isize, isize)) -> isize {
    (head.0 - tail.0).abs()
}

fn distance_y(head: (isize, isize), tail: (isize, isize)) -> isize {
    (head.1 - tail.1).abs()
}

fn must_move(head: (isize, isize), tail: (isize, isize)) -> bool {
    distance_x(head, tail) >= 2 || distance_y(head, tail) >= 2
}

fn parse_lines(lines: Vec<String>) -> Vec<(Move, usize)> {
    lines.iter().map(|l| parse_line(l)).collect::<Vec<_>>()
}

fn problem(moves: &Vec<(Move, usize)>, knots: usize) {
    let mut knots_pos: Vec<(isize, isize)> = (0..knots).map(|_| (0, 0)).collect();
    let mut tail_visited: HashMap<(isize, isize), isize> = HashMap::new();
    tail_visited.insert((0, 0), 1);
    for (mov, amount) in moves {
        let head_moves = match mov {
            Move::Down => (0..*amount)
                .map(|_| (0, -1))
                .collect::<Vec<(isize, isize)>>(),
            Move::Up => (0..*amount)
                .map(|_| (0, 1))
                .collect::<Vec<(isize, isize)>>(),
            Move::Left => (0..*amount)
                .map(|_| (-1, 0))
                .collect::<Vec<(isize, isize)>>(),
            Move::Right => (0..*amount)
                .map(|_| (1, 0))
                .collect::<Vec<(isize, isize)>>(),
            Move::Invalid => (0..*amount)
                .map(|_| (0, 0))
                .collect::<Vec<(isize, isize)>>(),
        };
        for head_move in head_moves {
            knots_pos[0] = (
                (knots_pos[0].0 + head_move.0),
                (knots_pos[0].1 + head_move.1),
            );
            for i in 1..knots_pos.len() {
                if must_move(knots_pos[i - 1], knots_pos[i]) {
                    knots_pos[i] = move_tail(knots_pos[i - 1], knots_pos[i]);
                    if i == knots_pos.len() - 1 {
                        tail_visited.insert(knots_pos[i], 1);
                    }
                }
            }
        }
    }
    let number_visited: isize = tail_visited.iter().map(|(_x, y)| y).sum();
    println!("Visited {0} locations", number_visited);
}

pub fn run() {
    let path = "./src/problem_9/input.txt".to_string();
    let f = File::open(path).unwrap();
    let lines = BufReader::new(f).lines();
    let moves = parse_lines(lines.flatten().collect());
    problem(&moves, 2);
    problem(&moves, 10);
}

#[test]
fn test_absolute_distance() {
    assert_eq!(distance_x((1, 0), (0, 0)), 1);
    assert_eq!(distance_x((2, 0), (0, 0)), 2);
    assert_eq!(distance_x((1, 0), (-1, 0)), 2);
    assert_eq!(distance_x((-1, 0), (1, 0)), 2);
    assert_eq!(distance_x((-1, 0), (-1, 0)), 0);
    assert_eq!(distance_y((0, 1), (0, 0)), 1);
    assert_eq!(distance_y((0, 2), (0, 0)), 2);
    assert_eq!(distance_y((0, 1), (0, -1)), 2);
    assert_eq!(distance_y((0, -1), (0, 1)), 2);
    assert_eq!(distance_y((0, -1), (0, -1)), 0);
}

#[test]
fn test_move_tail() {
    let mut tail = (0, 0);
    tail = move_tail((2, 0), tail);
    assert_eq!(tail.0, 1);
    assert_eq!(tail.1, 0);
    tail = move_tail((2, 2), tail);
    assert_eq!(tail.0, 2);
    assert_eq!(tail.1, 1);
}

#[test]
fn test_must_move() {
    assert_eq!(must_move((0, 0), (0, 0)), false);
    assert_eq!(must_move((1, 0), (0, 0)), false);
    assert_eq!(must_move((2, 0), (0, 0)), true);
    assert_eq!(must_move((2, 0), (1, 0)), false);
    assert_eq!(must_move((2, 1), (1, 0)), false);
    assert_eq!(must_move((2, 2), (1, 0)), true);
}
