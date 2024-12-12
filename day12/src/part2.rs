use std::{
    collections::{HashMap, HashSet, VecDeque},
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
    let mut sides: HashMap<(usize, Facing), HashSet<usize>> = HashMap::new();

    queue.push_back((start_x, start_y));

    while queue.len() > 0 {
        let pos = queue.pop_front().unwrap();
        visited.insert((pos.0, pos.1));

        if pos.0 > 0 && grid[pos.1][pos.0 - 1] == grid[pos.1][pos.0] {
            if !visited.contains(&(pos.0 - 1, pos.1)) {
                queue.push_back((pos.0 - 1, pos.1));
            }
        } else {
            let s = sides.get(&(pos.0, Facing::LEFT));
            if s.is_some() {
                let mut m = s.unwrap().to_owned();
                m.insert(pos.1);
                sides.insert((pos.0, Facing::LEFT), m.to_owned());
            } else {
                let m = HashSet::from_iter(vec![pos.1]);
                sides.insert((pos.0, Facing::LEFT), m.to_owned());
            }
        }

        if pos.0 < grid[pos.1].len() - 1 && grid[pos.1][pos.0 + 1] == grid[pos.1][pos.0] {
            if !visited.contains(&(pos.0 + 1, pos.1)) {
                queue.push_back((pos.0 + 1, pos.1));
            }
        } else {
            let s = sides.get(&(pos.0, Facing::RIGHT));
            if s.is_some() {
                let mut m = s.unwrap().to_owned();
                m.insert(pos.1);
                sides.insert((pos.0, Facing::RIGHT), m.to_owned());
            } else {
                let m = HashSet::from_iter(vec![pos.1]);
                sides.insert((pos.0, Facing::RIGHT), m.to_owned());
            }
        }

        if pos.1 > 0 && grid[pos.1 - 1][pos.0] == grid[pos.1][pos.0] {
            if !visited.contains(&(pos.0, pos.1 - 1)) {
                queue.push_back((pos.0, pos.1 - 1));
            }
        } else {
            let s = sides.get(&(pos.1, Facing::DOWN));
            if s.is_some() {
                let mut m = s.unwrap().to_owned();
                m.insert(pos.0);
                sides.insert((pos.1, Facing::DOWN), m.to_owned());
            } else {
                let m = HashSet::from_iter(vec![pos.0]);
                sides.insert((pos.1, Facing::DOWN), m.to_owned());
            }
        }

        if pos.1 < grid.len() - 1 && grid[pos.1 + 1][pos.0] == grid[pos.1][pos.0] {
            if !visited.contains(&(pos.0, pos.1 + 1)) {
                queue.push_back((pos.0, pos.1 + 1));
            }
        } else {
            let s = sides.get(&(pos.1, Facing::UP));
            if s.is_some() {
                let mut m = s.unwrap().to_owned();
                m.insert(pos.0);
                sides.insert((pos.1, Facing::UP), m.to_owned());
            } else {
                let m = HashSet::from_iter(vec![pos.0]);
                sides.insert((pos.1, Facing::UP), m.to_owned());
            }
        }
    }

    (sides.iter().map(|s| actual_sides(s.1)).sum(), visited)
}

fn actual_sides(s: &HashSet<usize>) -> usize {
    let mut v = s.iter().map(|i| i.to_owned()).collect::<Vec<usize>>();
    if v.len() < 2 {
        1
    } else {
        v.sort();
        let mut sides = 1;
        for i in 1..v.len() {
            if v[i] > v[i - 1] + 1 {
                sides += 1
            }
        }
        sides
    }
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 1206);
    }
}
