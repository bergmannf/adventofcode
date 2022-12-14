use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Grid {
    grid: Vec<Vec<u32>>,
}
impl Grid {
    fn columns(&self) -> usize {
        self.grid[0].len()
    }

    fn rows(&self) -> usize {
        self.grid.len()
    }

    fn print(&self) -> () {
        for i in 0..self.rows() {
            let mut line = String::new();
            for j in 0..self.columns() {
                if self.visible((i, j)) {
                    line.push('O');
                } else {
                    line.push('X');
                }
            }
            println!("{0}", line);
        }
    }

    fn columns_shorter(&self, location: (usize, usize)) -> bool {
        let height = self.grid[location.0][location.1];
        let top = (0..location.1).all(|i| self.grid[location.0][i] < height);
        let bottom = (location.1 + 1..self.columns()).all(|i| self.grid[location.0][i] < height);
        top || bottom
    }

    fn columns_scenic(&self, location: (usize, usize)) -> (usize, usize) {
        let height = self.grid[location.0][location.1];
        let mut score_top = 0;
        let mut score_bottom = 0;
        for j in (0..location.1).rev() {
            score_top += 1;
            if self.grid[location.0][j] >= height {
                break;
            }
        }
        for j in location.1 + 1..self.columns() {
            score_bottom += 1;
            if self.grid[location.0][j] >= height {
                break;
            }
        }
        (score_top, score_bottom)
    }

    fn rows_shorter(&self, location: (usize, usize)) -> bool {
        let height = self.grid[location.0][location.1];
        let left = (0..location.0).all(|i| self.grid[i][location.1] < height);
        let right = (location.0 + 1..self.rows()).all(|i| self.grid[i][location.1] < height);
        left || right
    }

    fn rows_scenic(&self, location: (usize, usize)) -> (usize, usize) {
        let height = self.grid[location.0][location.1];
        let mut score_left = 0;
        let mut score_right = 0;
        for i in (0..location.0).rev() {
            score_left += 1;
            if self.grid[i][location.1] >= height {
                break;
            }
        }
        for i in location.0 + 1..self.rows() {
            score_right += 1;
            if self.grid[i][location.1] >= height {
                break;
            }
        }
        (score_left, score_right)
    }

    fn scenic(&self, location: (usize, usize)) -> usize {
        let (top, bottom) = self.columns_scenic(location);
        let (left, right) = self.rows_scenic(location);
        top * bottom * left * right
    }

    fn visible(&self, location: (usize, usize)) -> bool {
        if location.0 == 0
            || location.1 == 0
            || location.0 == self.rows() - 1
            || location.1 == self.columns() - 1
        {
            return true;
        }
        self.columns_shorter(location) || self.rows_shorter(location)
    }
}

fn parse_grid(input: Vec<String>) -> Grid {
    let mut grid = Vec::new();
    for line in input {
        let sizes: Vec<u32> = line
            .chars()
            .into_iter()
            .map(|tree| tree.to_digit(10).unwrap())
            .collect();
        grid.push(sizes);
    }
    Grid { grid }
}

pub fn run() {
    let path = "./src/problem_8/input.txt";
    let input = File::open(path).unwrap();
    let grid = parse_grid(BufReader::new(input).lines().flatten().collect());
    grid.print();
    let mut visible_nodes = 0;
    for i in 0..grid.rows() {
        for j in 0..grid.columns() {
            if grid.visible((i, j)) {
                visible_nodes += 1;
            }
        }
    }
    println!("{0} location are visible", visible_nodes);
    let mut current_max = 0;
    for i in 0..grid.rows() {
        for j in 0..grid.columns() {
            let scenic = grid.scenic((i, j));
            if scenic > current_max {
                current_max = scenic;
            }
        }
    }
    println!("Best tree around {0}", current_max);
}

#[test]
fn test_parsing() {
    let input = vec!["012".to_owned(), "345".to_owned(), "678".to_owned()];
    let grid = parse_grid(input);
    assert_eq!(grid.grid[0][0], 0);
    assert_eq!(grid.grid[2][2], 8);
}

#[test]
fn test_not_visible() {
    let input = vec!["012".to_owned(), "345".to_owned(), "678".to_owned()];
    let grid = parse_grid(input);
    assert_eq!(grid.visible((1, 1)), false)
}

#[test]
fn test_visible() {
    let input = vec!["000".to_owned(), "010".to_owned(), "000".to_owned()];
    let grid = parse_grid(input);
    assert_eq!(grid.visible((1, 1)), true)
}
