use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use crate::part1::{find_trailheads, parse_input};

pub fn solve(path: &str) -> usize {
    let raw = fs::read_to_string(path).unwrap();
    let grid = parse_input(&raw);
    let trailheads = find_trailheads(&grid);
    trailheads
        .into_iter()
        .map(|t| trailhead_score(&grid, t.0, t.1))
        .sum()
}

fn trailhead_score(grid: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    if grid[y][x] == 9 {
        1
    } else {
        let mut total = 0;

        if x > 0 && grid[y][x - 1] == grid[y][x] + 1 {
            total += trailhead_score(grid, x - 1, y);
        }

        if x < grid[y].len() - 1 && grid[y][x + 1] == grid[y][x] + 1 {
            total += trailhead_score(grid, x + 1, y);
        }

        if y > 0 && grid[y - 1][x] == grid[y][x] + 1 {
            total += trailhead_score(grid, x, y - 1);
        }

        if y < grid.len() - 1 && grid[y + 1][x] == grid[y][x] + 1 {
            total += trailhead_score(grid, x, y + 1);
        }

        total
    }
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 81);
    }
}
