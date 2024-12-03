use std::fs;

use regex::Regex;

const ENABLE: &str = "do()";
const DISABLE: &str = "don't()";

pub fn solve(path: &str) -> u32 {
    let input = fs::read_to_string(path).unwrap();

    let mut writeable = true;
    let mut total = 0;

    let extract = Regex::new(r"^mul\((?<a>[0-9]{1,3}),(?<b>[0-9]{1,3})\)").unwrap();

    for i in 0..input.len() {
        let sub = input[i..input.len()].to_owned();
        if sub.starts_with(DISABLE) {
            writeable = false;
            continue;
        }

        if sub.starts_with(ENABLE) {
            writeable = true;
            continue;
        }

        if writeable && extract.find(sub.as_str()).is_some() {
            let cap = extract.captures(sub.as_str()).unwrap();
            let a = cap.name("a").unwrap().as_str().parse::<u32>().unwrap();
            let b = cap.name("b").unwrap().as_str().parse::<u32>().unwrap();
            total += a * b;
        }
    }
    total
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample2.txt"), 48)
    }
}
