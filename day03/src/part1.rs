use std::fs;

use regex::Regex;

pub fn solve(path: &str) -> u32 {
    let re = Regex::new(r"mul\([0-9]{1,3},[0-9]{1,3}\)").unwrap();
    let hay = fs::read_to_string(path).unwrap();
    re.find_iter(&hay).map(|m| m.as_str()).map(parse_mul).sum()
}

pub fn parse_mul(mul: &str) -> u32 {
    let re = Regex::new(r"mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").unwrap();
    let cap: Vec<(&str, &str)> = re
        .captures_iter(mul)
        .map(|m| {
            let a = m.name("a").unwrap().as_str();
            let b = m.name("b").unwrap().as_str();
            (a, b)
        })
        .collect();
    let a = cap[0].0.parse::<u32>().unwrap();
    let b = cap[0].1.parse::<u32>().unwrap();
    a * b
}

#[cfg(test)]
mod test {
    use crate::part1::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 161);
    }
}
