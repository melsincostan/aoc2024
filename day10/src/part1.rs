use std::{
    collections::{HashSet, VecDeque},
    fs,
};

pub fn solve(path: &str) -> usize {
    let raw = fs::read_to_string(path).unwrap();
    let grid = parse_input(&raw);
    let trailheads = find_trailheads(&grid);
    trailheads
        .into_iter()
        .map(|t| trailhead_score(&grid, t.0, t.1))
        .sum()
}

pub fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

pub fn find_trailheads(grid: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = vec![];
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 0 {
                res.push((x, y));
            }
        }
    }
    res
}

fn trailhead_score(grid: &Vec<Vec<u32>>, start_x: usize, start_y: usize) -> usize {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut destinations: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((start_x, start_y));

    while queue.len() > 0 {
        let visit = queue.pop_front().unwrap();
        visited.insert((visit.0, visit.1));
        let x = visit.0;
        let y = visit.1;
        let c = grid[y][x];

        if c == 9 {
            destinations.insert((x, y));
            continue;
        }

        if x > 0 && grid[y][x - 1] == c + 1 && !visited.contains(&(x - 1, y)) {
            queue.push_back((x - 1, y));
        }

        if x < grid[y].len() - 1 && grid[y][x + 1] == c + 1 && !visited.contains(&(x + 1, y)) {
            queue.push_back((x + 1, y));
        }

        if y > 0 && grid[y - 1][x] == c + 1 && !visited.contains(&(x, y - 1)) {
            queue.push_back((x, y - 1));
        }

        if y < grid.len() - 1 && grid[y + 1][x] == c + 1 && !visited.contains(&(x, y + 1)) {
            queue.push_back((x, y + 1));
        }
    }

    destinations.len()
}

#[cfg(test)]
mod test {
    use crate::part1::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 36);
    }
}
