use core::panic;
use std::{
    collections::{HashMap, VecDeque},
    fs,
};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
enum Facing {
    UP,    // north
    DOWN,  // south
    LEFT,  // west
    RIGHT, // east
}

pub fn solve(path: &str) -> u64 {
    let grid = parse(fs::read_to_string(path).unwrap());
    let start = find_start(&grid);
    *search(&grid, start.0, start.1, Facing::RIGHT)
        .iter()
        .min()
        .unwrap()
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'S' {
                return (x, y);
            }
        }
    }
    panic!("no start found");
}

fn parse(input: String) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

fn search(grid: &Vec<Vec<char>>, start_x: usize, start_y: usize, start_facing: Facing) -> Vec<u64> {
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 45);
    }
}
