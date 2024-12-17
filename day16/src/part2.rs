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
    let mut visited: HashMap<(usize, usize, Facing), u64> = HashMap::new();
    let mut scores: Vec<u64> = vec![];
    let mut queue: VecDeque<(usize, usize, u64, Facing)> = VecDeque::new();

    queue.push_back((start_x, start_y, 0, start_facing));

    let mut curr: (usize, usize, u64, Facing);
    while queue.len() > 0 {
        curr = queue.pop_front().unwrap();

        if grid[curr.1][curr.0] == 'E' {
            scores.push(curr.2);
            continue;
        }

        visited.insert((curr.0, curr.1, curr.3), curr.2);
        match curr.3 {
            Facing::UP => {
                if curr.1 > 0
                    && grid[curr.1 - 1][curr.0] != '#'
                    && visited
                        .get(&(curr.0, curr.1 - 1, Facing::UP))
                        .is_none_or(|v| *v > curr.2 + 1)
                {
                    queue.push_back((curr.0, curr.1 - 1, curr.2 + 1, Facing::UP));
                }

                if visited
                    .get(&(curr.0, curr.1, Facing::LEFT))
                    .is_none_or(|v| *v > curr.2 + 1000)
                {
                    queue.push_back((curr.0, curr.1, curr.2 + 1000, Facing::LEFT));
                }

                if visited
                    .get(&(curr.0, curr.1, Facing::RIGHT))
                    .is_none_or(|v| *v > curr.2 + 1000)
                {
                    queue.push_back((curr.0, curr.1, curr.2 + 1000, Facing::RIGHT));
                }
            }
            Facing::DOWN => {
                if curr.1 < grid.len() - 1
                    && grid[curr.1 + 1][curr.0] != '#'
                    && visited
                        .get(&(curr.0, curr.1 + 1, Facing::DOWN))
                        .is_none_or(|v| *v > curr.2 + 1)
                {
                    queue.push_back((curr.0, curr.1 + 1, curr.2 + 1, Facing::DOWN));
                }

                if visited
                    .get(&(curr.0, curr.1, Facing::LEFT))
                    .is_none_or(|v| *v > curr.2 + 1000)
                {
                    queue.push_back((curr.0, curr.1, curr.2 + 1000, Facing::LEFT));
                }

                if visited
                    .get(&(curr.0, curr.1, Facing::RIGHT))
                    .is_none_or(|v| *v > curr.2 + 1000)
                {
                    queue.push_back((curr.0, curr.1, curr.2 + 1000, Facing::RIGHT));
                }
            }
            Facing::LEFT => {
                if curr.0 > 0
                    && grid[curr.1][curr.0 - 1] != '#'
                    && visited
                        .get(&(curr.0 - 1, curr.1, Facing::LEFT))
                        .is_none_or(|v| *v > curr.2 + 1)
                {
                    queue.push_back((curr.0 - 1, curr.1, curr.2 + 1, Facing::LEFT));
                }

                if visited
                    .get(&(curr.0, curr.1, Facing::UP))
                    .is_none_or(|v| *v > curr.2 + 1000)
                {
                    queue.push_back((curr.0, curr.1, curr.2 + 1000, Facing::UP));
                }

                if visited
                    .get(&(curr.0, curr.1, Facing::DOWN))
                    .is_none_or(|v| *v > curr.2 + 1000)
                {
                    queue.push_back((curr.0, curr.1, curr.2 + 1000, Facing::DOWN));
                }
            }
            Facing::RIGHT => {
                if curr.0 < grid[curr.1].len() - 1
                    && grid[curr.1][curr.0 + 1] != '#'
                    && visited
                        .get(&(curr.0 + 1, curr.1, Facing::RIGHT))
                        .is_none_or(|v| *v > curr.2 + 1)
                {
                    queue.push_back((curr.0 + 1, curr.1, curr.2 + 1, Facing::RIGHT));
                }

                if visited
                    .get(&(curr.0, curr.1, Facing::UP))
                    .is_none_or(|v| *v > curr.2 + 1000)
                {
                    queue.push_back((curr.0, curr.1, curr.2 + 1000, Facing::UP));
                }

                if visited
                    .get(&(curr.0, curr.1, Facing::DOWN))
                    .is_none_or(|v| *v > curr.2 + 1000)
                {
                    queue.push_back((curr.0, curr.1, curr.2 + 1000, Facing::DOWN));
                }
            }
        }
    }
    scores
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 45);
    }
}
