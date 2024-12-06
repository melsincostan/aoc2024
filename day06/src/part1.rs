use std::{collections::HashSet, fs};

enum Facing {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub fn solve(path: &str) -> usize {
    let grid: Vec<Vec<char>> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let gr_y = grid.len();
    let gr_x = grid[0].len();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let start = start_pos(&grid).unwrap();
    let mut dir = start.2;
    let mut pos = (start.0, start.1);
    loop {
        visited.insert(pos);
        let next = next_pos(pos.0, pos.1, gr_x, gr_y, &dir);
        if next.is_none() {
            break;
        }

        let u = next.unwrap();
        if grid[u.1][u.0] == '#' {
            dir = rotate_dir_90(&dir);
        } else {
            pos = (u.0, u.1)
        }
    }
    visited.len()
}

fn rotate_dir_90(dir: &Facing) -> Facing {
    match dir {
        Facing::UP => Facing::RIGHT,
        Facing::RIGHT => Facing::DOWN,
        Facing::DOWN => Facing::LEFT,
        Facing::LEFT => Facing::UP,
    }
}

fn start_pos(grid: &Vec<Vec<char>>) -> Option<(usize, usize, Facing)> {
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            match grid[y][x] {
                '^' => return Some((x, y, Facing::UP)),
                _ => continue,
            }
        }
    }
    None
}

fn next_pos(x: usize, y: usize, mx: usize, my: usize, facing: &Facing) -> Option<(usize, usize)> {
    match facing {
        Facing::UP => {
            if y > 0 {
                Some((x, y - 1))
            } else {
                None
            }
        }
        Facing::DOWN => {
            if y < my - 1 {
                Some((x, y + 1))
            } else {
                None
            }
        }
        Facing::LEFT => {
            if x > 0 {
                Some((x - 1, y))
            } else {
                None
            }
        }
        Facing::RIGHT => {
            if x < mx - 1 {
                Some((x + 1, y))
            } else {
                None
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::part1::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 41);
    }
}
