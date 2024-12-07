use std::collections::VecDeque;

pub fn solve(path: &str) -> u32 {
    0
}

fn line_solvable(line: &VecDeque<u32>, acc: u32, target: u32) -> bool {
    if line.len() < 1 {
        acc == target
    } else if acc >= target {
        false
    } else {
        let mut nl = line.clone();
        let next = nl.pop_front().unwrap();
        line_solvable(&nl, acc + next, target) || line_solvable(&nl, acc * next, target)
    }
}

#[cfg(test)]
mod test {
    use std::collections::VecDeque;

    use crate::part1::{line_solvable, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 3749);
    }

    #[test]
    fn test_line_solveable() {
        let a = VecDeque::from_iter(vec![19]);
        let b = VecDeque::from_iter(vec![40, 27]);
        let c = VecDeque::from_iter(vec![6, 16, 20]);
        let d = VecDeque::from_iter(vec![10, 13]);
        assert_eq!(line_solvable(&a, 10, 190), true);
        assert_eq!(line_solvable(&b, 81, 3267), true);
        assert_eq!(line_solvable(&c, 11, 292), true);
        assert_eq!(line_solvable(&d, 16, 161011), false);
    }
}
