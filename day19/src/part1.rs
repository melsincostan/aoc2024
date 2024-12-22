use std::fs;

pub fn solve(path: &str) -> usize {
    let raw = fs::read_to_string(path).unwrap();
    let (towels, patterns) = parse(raw.as_str());
    patterns.iter().filter(|p| possible(&towels, p)).count()
}

fn parse(input: &str) -> (Vec<&str>, Vec<&str>) {
    let spl: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(spl.len(), 2);
    let towels = parse_towels(spl[0]);
    let patterns = parse_patterns(spl[1]);
    (towels, patterns)
}

fn parse_towels(input: &str) -> Vec<&str> {
    input.split(",").map(|t| t.trim()).collect()
}

fn parse_patterns(input: &str) -> Vec<&str> {
    input.lines().map(|l| l.trim()).collect()
}

pub fn possible(towels: &Vec<&str>, pattern: &str) -> bool {
    if pattern.len() < 1 {
        return true;
    }
    let mut res = false;
    for i in 0..towels.len() {
        if pattern.starts_with(towels[i]) {
            res = res || possible(towels, &pattern[towels[i].len()..]);
            if res {
                break; // no need to go further since we only want to know whether it is possible or not
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use crate::part1::{possible, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 6);
    }

    #[test]
    fn test_possible() {
        assert_eq!(possible(&vec!["rg", "r"], "rgrgr"), true);
        assert_eq!(possible(&vec!["rgr", "r", "rg"], "rgrgr"), true);
        assert_eq!(possible(&vec!["rgr", "grgr"], "rgrgr"), false);
    }
}
