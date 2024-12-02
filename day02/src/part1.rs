use std::fs;

pub fn solve(path: &str) -> usize {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(parse_line)
        .filter(check_line)
        .count()
}

fn parse_line(line: &str) -> Vec<u32> {
    line.split_whitespace()
        .map(|entry: &str| entry.parse::<u32>().unwrap())
        .collect()
}

fn check_line(line: &Vec<u32>) -> bool {
    false
}

#[cfg(test)]
mod test {
    use crate::part1::{parse_line, solve};

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
}
