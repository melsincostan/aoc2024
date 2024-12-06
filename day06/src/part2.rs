use std::{collections::HashSet, fs};

use crate::part1::{self, next_pos, start_pos, Facing};

pub fn solve(path: &str) -> u32 {
    let grid: Vec<Vec<char>> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let start = start_pos(&grid).unwrap();
    let orig_path = grid_positions(&grid, start.0, start.1, start.2.to_owned());
    orig_path
        .0
        .into_iter()
        .filter(|p| p.0 != start.0 || p.1 != start.1)
        .map(|c| {
            let mut ng = grid.to_owned();
            ng[c.1][c.0] = 'O';
            let res = grid_positions(&ng, start.0, start.1, start.2.to_owned());
            if res.1 {
                1
            } else {
                0
            }
        })
        .sum()
}

fn grid_positions(
    grid: &Vec<Vec<char>>,
    sx: usize,
    sy: usize,
    sd: Facing,
) -> (HashSet<(usize, usize)>, bool) {
    let mut dir: Facing = sd;
    let mut pos = (sx, sy);
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut went: HashSet<(usize, usize, Facing)> = HashSet::new();
    let mut cycles = false;
    let gr_y = grid.len();
    let gr_x = grid[0].len();
    loop {
        visited.insert(pos);
        went.insert((pos.0, pos.1, dir.clone()));
        let next = next_pos(pos.0, pos.1, gr_x, gr_y, &dir);
        if next.is_none() {
            break;
        }

        let u = next.unwrap();
        if grid[u.1][u.0] == '#' || grid[u.1][u.0] == 'O' {
            dir = part1::rotate_dir_90(&dir);
        } else {
            pos = (u.0, u.1);
            if went.contains(&(pos.0, pos.1, dir)) {
                cycles = true;
                break;
            }
        }
    }
    (visited, cycles)
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 6);
    }
}
