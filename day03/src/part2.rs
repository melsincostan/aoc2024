use std::fs;

use regex::Regex;

use crate::part1;

pub fn solve(path: &str) -> u32 {
    let input = fs::read_to_string(path).unwrap();
    let refp = Regex::new(r"don't\(\).*?do\(\)").unwrap();
    let fp = refp.replace_all(&input, "");
    // first pass would leave any final don't() with nothing getting re-enabled after in.
    let resp = Regex::new(r"don't\(\).*").unwrap();
    let sp = resp.replace_all(&fp, "");
    let re = Regex::new(r"mul\([1-9]{1,3},[1-9]{1,3}\)").unwrap();
    re.find_iter(&sp)
        .map(|m| m.as_str())
        .map(part1::parse_mul)
        .sum()
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample2.txt"), 48)
    }
}
