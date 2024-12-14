pub fn solve(path: &str) -> u32 {
    0
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

#[cfg(test)]
mod test {
    use crate::part1::{parse_robot, parse_set, pos_in, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 0);
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
