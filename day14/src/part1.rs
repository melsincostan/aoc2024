use std::fs;

pub fn solve(path: &str) -> u64 {
    let robots: Vec<(i64, i64)> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| parse_robot(l))
        .map(|r| pos_in(100, r.0 .0, r.0 .1, r.1 .0, r.1 .1, 101, 103))
        .collect();
    safety_factor(&robots, 101, 103)
}

pub fn pos_in(
    steps: i64,
    start_x: i64,
    start_y: i64,
    vel_x: i64,
    vel_y: i64,
    grid_x: i64,
    grid_y: i64,
) -> (i64, i64) {
    let mut x = (start_x + steps * vel_x) % grid_x;
    let mut y = (start_y + steps * vel_y) % grid_y;
    if x < 0 {
        x = grid_x - (x.abs() % grid_x);
    }

    if y < 0 {
        y = grid_y - (y.abs() % grid_y);
    }
    (x, y)
}

fn parse_robot(input: &str) -> ((i64, i64), (i64, i64)) {
    let raw_sets: Vec<&str> = input.split(" ").collect();
    assert_eq!(raw_sets.len(), 2);
    let start = parse_set(raw_sets[0]);
    let velocity = parse_set(raw_sets[1]);
    (start, velocity)
}

fn parse_set(input: &str) -> (i64, i64) {
    let raw_coords: Vec<&str> = input[2..input.len()].split(",").collect();
    assert_eq!(raw_coords.len(), 2);
    let x = raw_coords[0].parse::<i64>().unwrap();
    let y = raw_coords[1].parse::<i64>().unwrap();
    (x, y)
}

fn safety_factor(robots: &Vec<(i64, i64)>, grid_x: i64, grid_y: i64) -> u64 {
    let lr_delim = grid_x / 2; // ceil
    let tb_delim = grid_y / 2; // ceil
    let mut tr = 0;
    let mut tl = 0;
    let mut br = 0;
    let mut bl = 0;
    robots.into_iter().for_each(|r| {
        if r.0 < lr_delim {
            if r.1 < tb_delim {
                tl += 1;
            } else if r.1 > tb_delim {
                bl += 1;
            }
        } else if r.0 > lr_delim {
            if r.1 < tb_delim {
                tr += 1;
            } else if r.1 > tb_delim {
                br += 1;
            }
        }
    });

    tr * tl * br * bl
}

#[cfg(test)]
mod test {
    use crate::part1::{parse_robot, parse_set, pos_in, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 12);
    }

    #[test]
    fn test_pos_in() {
        assert_eq!(pos_in(0, 1, 1, 2, 2, 5, 5), (1, 1));
        assert_eq!(pos_in(1, 2, 4, 2, -3, 11, 7), (4, 1));
        assert_eq!(pos_in(2, 2, 4, 2, -3, 11, 7), (6, 5));
        assert_eq!(pos_in(3, 2, 4, 2, -3, 11, 7), (8, 2));
        assert_eq!(pos_in(4, 2, 4, 2, -3, 11, 7), (10, 6));
        assert_eq!(pos_in(5, 2, 4, 2, -3, 11, 7), (1, 3));
    }

    #[test]
    fn test_parse_set() {
        assert_eq!(parse_set("v=3,-3"), (3, -3));
    }

    #[test]
    fn test_parse_robot() {
        assert_eq!(parse_robot("p=6,3 v=-1,-3"), ((6, 3), (-1, -3)));
    }
}
