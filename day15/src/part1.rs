use core::panic;
use std::{collections::VecDeque, fs};

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum GridCell {
    WALL,
    CRATE,
    VOID,
}

pub fn solve(path: &str) -> usize {
    let raw = fs::read_to_string(path).unwrap();
    let input: Vec<&str> = raw.split("\n\n").collect();
    assert_eq!(input.len(), 2);
    let mut grid = parse_grid(input[0]);
    let directions = parse_directions(input[1]);
    let start_pos = find_robot(input[0]);
    let mut x = start_pos.0;
    let mut y = start_pos.1;
    for i in 0..directions.len() {
        let res = do_move(&mut grid, &directions[i], x, y);
        x = res.0;
        y = res.1;
    }
    let mut res = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == GridCell::CRATE {
                res += gps_value(x, y);
            }
        }
    }
    res
}

fn dir(input: char) -> Option<Direction> {
    match input {
        '>' => Some(Direction::RIGHT),
        '<' => Some(Direction::LEFT),
        '^' => Some(Direction::UP),
        'v' => Some(Direction::DOWN),
        _ => None,
    }
}

fn obj(input: char) -> GridCell {
    match input {
        '#' => GridCell::WALL,
        'O' => GridCell::CRATE,
        _ => GridCell::VOID,
    }
}

fn find_robot(input: &str) -> (usize, usize) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '@' {
                return (x, y);
            }
        }
    }
    panic!("robot not found");
}

fn parse_grid(input: &str) -> Vec<Vec<GridCell>> {
    input
        .lines()
        .map(|l| l.chars().map(obj).collect())
        .collect()
}

fn parse_directions(input: &str) -> VecDeque<Direction> {
    VecDeque::from_iter(
        input
            .chars()
            .map(dir)
            .filter(|o| o.is_some())
            .map(|o| o.unwrap()),
    )
}

fn do_move(
    grid: &mut Vec<Vec<GridCell>>,
    direction: &Direction,
    start_x: usize,
    start_y: usize,
) -> (usize, usize) {
    match direction {
        Direction::UP => {
            let mut crates = 0;
            let mut until_y = start_y;
            loop {
                until_y -= 1;
                match grid[until_y][start_x] {
                    GridCell::WALL => {
                        return (start_x, start_y); // can't move
                    }
                    GridCell::CRATE => crates += 1, // add crate to the stack to push
                    GridCell::VOID => break, // as soon as one cell is free, we know we can move
                }
            }
            // if we get here: we can move, eventually shoving crates
            // move the crates
            for i in 0..crates {
                grid[start_y - (i + 2)][start_x] = GridCell::CRATE;
            }
            // set the new position to be a void to not duplicate crates...
            grid[start_y - 1][start_x] = GridCell::VOID;
            // return the new position
            (start_x, start_y - 1)
        }
        Direction::DOWN => {
            let mut crates = 0;
            let mut until_y = start_y;
            loop {
                until_y += 1;
                match grid[until_y][start_x] {
                    GridCell::WALL => {
                        return (start_x, start_y); // can't move
                    }
                    GridCell::CRATE => crates += 1, // add crate to the stack to push
                    GridCell::VOID => break, // as soon as one cell is free, we know we can move
                }
            }
            // if we get here: we can move, eventually shoving crates
            // move the crates
            for i in 0..crates {
                grid[start_y + (i + 2)][start_x] = GridCell::CRATE;
            }
            // set the new position to be a void to not duplicate crates...
            grid[start_y + 1][start_x] = GridCell::VOID;

            // return the new position
            (start_x, start_y + 1)
        }
        Direction::LEFT => {
            let mut crates = 0;
            let mut until_x = start_x;
            loop {
                until_x -= 1;
                match grid[start_y][until_x] {
                    GridCell::WALL => return (start_x, start_y),
                    GridCell::CRATE => crates += 1,
                    GridCell::VOID => break,
                }
            }
            for i in 0..crates {
                grid[start_y][start_x - (i + 2)] = GridCell::CRATE;
            }

            // set the new position to be a void to not duplicate crates...
            grid[start_y][start_x - 1] = GridCell::VOID;

            (start_x - 1, start_y)
        }
        Direction::RIGHT => {
            let mut crates = 0;
            let mut until_x = start_x;
            loop {
                until_x += 1;
                match grid[start_y][until_x] {
                    GridCell::WALL => return (start_x, start_y),
                    GridCell::CRATE => crates += 1,
                    GridCell::VOID => break,
                }
            }
            for i in 0..crates {
                grid[start_y][start_x + (i + 2)] = GridCell::CRATE;
            }
            // set the new position to be a void to not duplicate crates...
            grid[start_y][start_x + 1] = GridCell::VOID;

            (start_x + 1, start_y)
        }
    }
}

fn gps_value(x: usize, y: usize) -> usize {
    x + 100 * y
}

#[cfg(test)]
mod test {
    use crate::part1::{dir, obj, solve, Direction, GridCell};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 2028);
        assert_eq!(solve("big_sample.txt"), 10092);
    }

    #[test]
    fn test_dir() {
        assert_eq!(dir('>'), Some(Direction::RIGHT));
        assert_eq!(dir('<'), Some(Direction::LEFT));
        assert_eq!(dir('^'), Some(Direction::UP));
        assert_eq!(dir('v'), Some(Direction::DOWN));
        assert_eq!(dir('a'), None);
        assert_eq!(dir('\n'), None);
    }

    #[test]
    fn test_obj() {
        assert_eq!(obj('#'), GridCell::WALL);
        assert_eq!(obj('O'), GridCell::CRATE);
        assert_eq!(obj('@'), GridCell::VOID);
        assert_eq!(obj('.'), GridCell::VOID);
    }
}
