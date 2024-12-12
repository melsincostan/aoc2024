use std::{
    collections::{HashSet, VecDeque},
    fs,
};

#[derive(Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Facing {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

pub fn solve(path: &str) -> usize {
    let grid = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    let mut attributed: HashSet<(usize, usize)> = HashSet::new();
    let mut plots: Vec<(usize, usize)> = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if attributed.contains(&(x, y)) {
                continue;
            }

            let res = area(&grid, x, y);
            plots.push((res.0, res.1.len()));
            attributed.extend(res.1.iter());
        }
    }
    plots.into_iter().map(|p| p.0 * p.1).sum()
}

fn area(grid: &Vec<Vec<char>>, start_x: usize, start_y: usize) -> (usize, HashSet<(usize, usize)>) {
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut perimeter: HashSet<(usize, usize, Facing)> = HashSet::new();

    queue.push_back((start_x, start_y));

    while queue.len() > 0 {
        let pos = queue.pop_front().unwrap();
        visited.insert((pos.0, pos.1));

        if pos.0 > 0 && grid[pos.1][pos.0 - 1] == grid[pos.1][pos.0] {
            if !visited.contains(&(pos.0 - 1, pos.1)) {
                queue.push_back((pos.0 - 1, pos.1));
            }
        } else {
            perimeter.insert((pos.0, pos.1, Facing::LEFT));
        }

        if pos.0 < grid[pos.1].len() - 1 && grid[pos.1][pos.0 + 1] == grid[pos.1][pos.0] {
            if !visited.contains(&(pos.0 + 1, pos.1)) {
                queue.push_back((pos.0 + 1, pos.1));
            }
        } else {
            perimeter.insert((pos.0, pos.1, Facing::RIGHT));
        }

        if pos.1 > 0 && grid[pos.1 - 1][pos.0] == grid[pos.1][pos.0] {
            if !visited.contains(&(pos.0, pos.1 - 1)) {
                queue.push_back((pos.0, pos.1 - 1));
            }
        } else {
            perimeter.insert((pos.0, pos.1, Facing::DOWN));
        }

        if pos.1 < grid.len() - 1 && grid[pos.1 + 1][pos.0] == grid[pos.1][pos.0] {
            if !visited.contains(&(pos.0, pos.1 + 1)) {
                queue.push_back((pos.0, pos.1 + 1));
            }
        } else {
            perimeter.insert((pos.0, pos.1, Facing::UP));
        }
    }

    (perimeter.len(), visited)
}

#[cfg(test)]
mod test {
    use crate::part1::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 1930);
    }
}
