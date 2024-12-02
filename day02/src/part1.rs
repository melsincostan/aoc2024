use std::fs;

pub fn solve(path: &str) -> usize {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(parse_line)
        .filter(check_line)
        .count()
}

fn parse_line(line: &str) -> Vec<i32> {
    line.split_whitespace()
        .map(|entry: &str| entry.parse::<i32>().unwrap())
        .collect()
}

fn check_line(line: &Vec<i32>) -> bool {
    if line.len() == 0 {
        return false; // skipping empty lines, not counting them
    }

    if line.len() == 1 {
        return true; // can't be false
    }

    if line[0] == line[1] {
        return false;
    }

    let asc = line[0] < line[1];

    for i in 1..(line.len() - 1) {
        let diff = line[i] - line[i + 1];

        if diff.abs() > 3 || diff.abs() < 1 {
            return false; // too big of a difference
        }

        if (asc && diff > 0) || (!asc && diff < 0) {
            return false; // not going the right way
        }
    }
    true
}

#[cfg(test)]
mod test {
    use crate::part1::{check_line, parse_line, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 2);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("1 2 3"), vec![1, 2, 3]);
        assert_eq!(parse_line("1"), vec![1]);
        assert_eq!(parse_line(" "), vec![])
    }

    #[test]
    #[should_panic]
    fn test_parse_line_invalid() {
        parse_line("a");
    }

    #[test]
    fn test_check_line() {
        assert_eq!(check_line(&vec![1, 2, 3]), true);
        assert_eq!(check_line(&vec![3, 2, 1]), true);
        assert_eq!(check_line(&vec![]), false);
        assert_eq!(check_line(&vec![1]), true);
        assert_eq!(check_line(&vec![1, 2, 3, 2]), false);
        assert_eq!(check_line(&vec![3, 2, 1, 2]), false);
        assert_eq!(check_line(&vec![1, 2, 3, 12]), false);
        assert_eq!(check_line(&vec![1, 2, 2, 3]), false);
    }
}
