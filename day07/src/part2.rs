use std::{collections::VecDeque, fs};

pub fn solve(path: &str) -> u64 {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| {
            let parts: Vec<&str> = l.split(": ").collect();
            assert_eq!(parts.len(), 2);
            let target = parts[0].parse::<u64>().unwrap();
            let inputs = VecDeque::from_iter(
                parts[1]
                    .split_whitespace()
                    .map(|r| r.parse::<u64>().unwrap()),
            );
            (target, inputs)
        })
        .filter(|i| {
            let mut nl = i.1.clone();
            let first = nl.pop_front().unwrap();
            line_solvable(&nl, first, i.0)
        })
        .map(|l| l.0)
        .sum()
}

fn line_solvable(line: &VecDeque<u64>, acc: u64, target: u64) -> bool {
    if line.len() < 1 {
        acc == target
    } else if acc > target {
        false
    } else {
        let mut nl = line.clone();
        let next = nl.pop_front().unwrap();
        line_solvable(&nl, acc + next, target)
            || line_solvable(&nl, acc * next, target)
            || line_solvable(&nl, concat(acc, next), target)
    }
}

fn concat(a: u64, b: u64) -> u64 {
    format!("{}{}", a, b).parse::<u64>().unwrap()
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::part2::{line_solvable, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 11387);
    }

    #[test]
    fn test_line_solveable() {
        let a = VecDeque::from_iter(vec![19, 0]);
        let b = VecDeque::from_iter(vec![40, 27]);
        let c = VecDeque::from_iter(vec![6, 16, 20]);
        let d = VecDeque::from_iter(vec![10, 13]);
        assert_eq!(line_solvable(&a, 10, 190), true);
        assert_eq!(line_solvable(&b, 81, 3267), true);
        assert_eq!(line_solvable(&c, 11, 292), true);
        assert_eq!(line_solvable(&d, 16, 161011), false);
    }
}
