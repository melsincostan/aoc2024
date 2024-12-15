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
    CRATE_LEFT,
    CRATE_RIGHT,
    VOID,
}

pub fn solve(path: &str) -> usize {
    let raw = fs::read_to_string(path).unwrap();
    let input: Vec<&str> = raw.split("\n\n").collect();
    assert_eq!(input.len(), 2);
    let mut grid = parse_grid(input[0]);
    println!("past parsing grid");
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
            if grid[y][x] == GridCell::CRATE_LEFT {
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

fn obj(input: char) -> Vec<GridCell> {
    match input {
        '#' => vec![GridCell::WALL, GridCell::WALL],
        'O' => vec![GridCell::CRATE_LEFT, GridCell::CRATE_RIGHT],
        _ => vec![GridCell::VOID, GridCell::VOID],
    }
}

fn find_robot(input: &str) -> (usize, usize) {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '@' {
                return (x * 2, y);
            }
        }
    }
    panic!("robot not found");
}

fn parse_grid(input: &str) -> Vec<Vec<GridCell>> {
    input
        .lines()
        .map(|l| l.chars().map(obj).flatten().collect())
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
        Direction::UP => match grid[start_y - 1][start_x] {
            GridCell::WALL => (start_x, start_y),
            GridCell::VOID => (start_x, start_y - 1),
            GridCell::CRATE_LEFT => {
                if can_move_crate(grid, start_x, start_y - 1, &Direction::UP) {
                    move_crate(grid, start_x, start_y - 1, &Direction::UP);
                    (start_x, start_y - 1)
                } else {
                    (start_x, start_y)
                }
            }
            GridCell::CRATE_RIGHT => {
                if can_move_crate(grid, start_x - 1, start_y - 1, &Direction::UP) {
                    move_crate(grid, start_x - 1, start_y - 1, &Direction::UP);
                    (start_x, start_y - 1)
                } else {
                    (start_x, start_y)
                }
            }
        },
        Direction::DOWN => match grid[start_y + 1][start_x] {
            GridCell::WALL => (start_x, start_y),
            GridCell::VOID => (start_x, start_y + 1),
            GridCell::CRATE_LEFT => {
                if can_move_crate(grid, start_x, start_y + 1, &Direction::DOWN) {
                    move_crate(grid, start_x, start_y + 1, &Direction::DOWN);
                    (start_x, start_y + 1)
                } else {
                    (start_x, start_y)
                }
            }
            GridCell::CRATE_RIGHT => {
                if can_move_crate(grid, start_x - 1, start_y + 1, &Direction::DOWN) {
                    move_crate(grid, start_x - 1, start_y + 1, &Direction::DOWN);
                    (start_x, start_y + 1)
                } else {
                    (start_x, start_y)
                }
            }
        },
        Direction::LEFT => match grid[start_y][start_x - 1] {
            GridCell::WALL => (start_x, start_y),
            GridCell::CRATE_LEFT => panic!("robot seems to be phasing through a crate. fix this."),
            GridCell::CRATE_RIGHT => {
                if can_move_crate(grid, start_x - 2, start_y, &Direction::LEFT) {
                    move_crate(grid, start_x - 2, start_y, &Direction::LEFT);
                    (start_x - 1, start_y)
                } else {
                    (start_x, start_y)
                }
            }
            GridCell::VOID => (start_x - 1, start_y),
        },
        Direction::RIGHT => match grid[start_y][start_x + 1] {
            GridCell::WALL => (start_x, start_y),
            GridCell::VOID => (start_x + 1, start_y),
            GridCell::CRATE_RIGHT => panic!("robot seems to be phasing through a crate. fix this."),
            GridCell::CRATE_LEFT => {
                if can_move_crate(grid, start_x + 1, start_y, &Direction::RIGHT) {
                    move_crate(grid, start_x + 1, start_y, &Direction::RIGHT);
                    (start_x + 1, start_y)
                } else {
                    (start_x, start_y)
                }
            }
        },
    }
}

fn can_move_crate(grid: &Vec<Vec<GridCell>>, lx: usize, ly: usize, direction: &Direction) -> bool {
    match direction {
        Direction::LEFT => match grid[ly][lx - 1] {
            GridCell::WALL => false,
            GridCell::VOID => true,
            GridCell::CRATE_RIGHT => can_move_crate(grid, lx - 2, ly, &Direction::LEFT),
            GridCell::CRATE_LEFT => panic!(
                "should not be possible unless there is overlap with another crate. fix this."
            ),
        },
        Direction::RIGHT => match grid[ly][lx + 2] {
            GridCell::WALL => false,
            GridCell::VOID => true,
            GridCell::CRATE_LEFT => can_move_crate(grid, lx + 2, ly, &Direction::RIGHT),
            GridCell::CRATE_RIGHT => panic!(
                "should not be possible unless there is overlap with another crate. fix this."
            ),
        },
        Direction::UP => {
            // this is where it gets complicated (:
            let l = match grid[ly - 1][lx] {
                GridCell::WALL => false,
                GridCell::VOID => true,
                GridCell::CRATE_LEFT => can_move_crate(grid, lx, ly - 1, &Direction::UP),
                GridCell::CRATE_RIGHT => can_move_crate(grid, lx - 1, ly - 1, &Direction::UP),
            };
            let r = match grid[ly - 1][lx + 1] {
                GridCell::WALL => false,
                GridCell::VOID => true,
                GridCell::CRATE_LEFT => can_move_crate(grid, lx + 1, ly - 1, &Direction::UP),
                GridCell::CRATE_RIGHT => true, // invalid crates will get caught by l, no need to go all the way through again
            };
            l && r
        }
        Direction::DOWN => {
            let l = match grid[ly + 1][lx] {
                GridCell::WALL => false,
                GridCell::VOID => true,
                GridCell::CRATE_LEFT => can_move_crate(grid, lx, ly + 1, &Direction::DOWN),
                GridCell::CRATE_RIGHT => can_move_crate(grid, lx - 1, ly + 1, &Direction::DOWN),
            };
            let r = match grid[ly + 1][lx + 1] {
                GridCell::WALL => false,
                GridCell::VOID => true,
                GridCell::CRATE_LEFT => can_move_crate(grid, lx + 1, ly + 1, &Direction::DOWN),
                GridCell::CRATE_RIGHT => true, // invalid crates will get caught by l, no need to go all the way through again
            };
            l && r
        }
    }
}

fn move_crate(grid: &mut Vec<Vec<GridCell>>, lx: usize, ly: usize, direction: &Direction) {
    match direction {
        Direction::LEFT => {
            match grid[ly][lx - 1] {
                GridCell::CRATE_RIGHT => move_crate(grid, lx - 2, ly, &Direction::LEFT),
                _ => (), // should be checked beforehand so no wall or whatever is there
            }
            grid[ly][lx + 1] = GridCell::VOID;
            grid[ly][lx] = GridCell::CRATE_RIGHT;
            grid[ly][lx - 1] = GridCell::CRATE_LEFT;
        }
        Direction::RIGHT => {
            match grid[ly][lx + 2] {
                GridCell::CRATE_LEFT => move_crate(grid, lx + 2, ly, &Direction::RIGHT),
                _ => (),
            }
            grid[ly][lx] = GridCell::VOID;
            grid[ly][lx + 1] = GridCell::CRATE_LEFT;
            grid[ly][lx + 2] = GridCell::CRATE_RIGHT;
        }
        Direction::UP => {
            // ;-;
            match grid[ly - 1][lx] {
                GridCell::CRATE_LEFT => move_crate(grid, lx, ly - 1, &Direction::UP),
                GridCell::CRATE_RIGHT => move_crate(grid, lx - 1, ly - 1, &Direction::UP),
                _ => (),
            }
            match grid[ly - 1][lx + 1] {
                GridCell::CRATE_LEFT => move_crate(grid, lx + 1, ly - 1, &Direction::UP),
                _ => (), // CRATE_RIGHT was already caught by the previous check as a CRATE_LEFT!
            }
            grid[ly][lx] = GridCell::VOID;
            grid[ly][lx + 1] = GridCell::VOID;
            grid[ly - 1][lx] = GridCell::CRATE_LEFT;
            grid[ly - 1][lx + 1] = GridCell::CRATE_RIGHT;
        }
        Direction::DOWN => {
            match grid[ly + 1][lx] {
                GridCell::CRATE_LEFT => move_crate(grid, lx, ly + 1, &Direction::DOWN),
                GridCell::CRATE_RIGHT => move_crate(grid, lx - 1, ly + 1, &Direction::DOWN),
                _ => (),
            }
            match grid[ly + 1][lx + 1] {
                GridCell::CRATE_LEFT => move_crate(grid, lx + 1, ly + 1, &Direction::DOWN),
                _ => (), // CRATE_RIGHT was already caught by the previous check as a CRATE_LEFT!
            }
            grid[ly][lx] = GridCell::VOID;
            grid[ly][lx + 1] = GridCell::VOID;
            grid[ly + 1][lx] = GridCell::CRATE_LEFT;
            grid[ly + 1][lx + 1] = GridCell::CRATE_RIGHT;
        }
    }
}

fn gps_value(x: usize, y: usize) -> usize {
    x + 100 * y
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("big_sample.txt"), 9021);
    }
}
