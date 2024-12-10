use std::collections::{HashSet, VecDeque};

pub fn solve(path: &str) -> u32 {
    0
}

fn trailhead_score(grid: &Vec<Vec<u8>>, start_x: usize, start_y: usize) -> usize {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut destinations: HashSet<(usize, usize)> = HashSet::new();

    queue.push_back((start_x, start_y));

    while queue.len() > 1 {
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
        assert_eq!(solve("sample.txt"), 0);
    }
}
