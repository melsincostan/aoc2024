use std::{cmp::Ordering, collections::HashMap, fs};

use crate::part1;

pub fn solve(path: &str) -> usize {
    let raw = fs::read_to_string(path).unwrap();
    let (mut towels, patterns) = parse(raw.as_str());
    let map = preprocess(&towels);
    println!("done preprocessing");
    towels.sort_by(|a, b| sort_func(a, b));
    patterns
        .iter()
        .filter(|p| part1::possible(&towels, p))
        .map(|p| {
            println!("pattern: {}", p);
            let res = amount(&map, &towels, p, "");
            println!("res: {}", res);
            res
        })
        .sum()
}

fn sort_func(a: &str, b: &str) -> Ordering {
    if b.len() > a.len() {
        Ordering::Greater
    } else if b.len() < a.len() {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

fn amount(map: &HashMap<String, usize>, towels: &Vec<&str>, pattern: &str, trace: &str) -> usize {
    if pattern.len() < 1 {
        print!("OK\n");
        return 1;
    }
    let mut res = 0;
    for i in 0..towels.len() {
        if pattern.starts_with(towels[i]) {
            print!("{}, ", towels[i]);
            let mut nt = trace.to_owned();
            res = map.get(towels[i]).unwrap().0 * amount(map, towels, &pattern[towels[i].len()..]);
            if res != 0 {
                break;
            }
        }
    }
    print!("\n");
    res
}

fn preprocess(towels: &Vec<&str>) -> HashMap<String, usize> {
    let mut map = HashMap::new();
    for i in 0..towels.len() {
        map.insert(towels[i].to_string(), amount_naive(towels, towels[i]));
    }
    map
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

fn amount_naive(towels: &Vec<&str>, pattern: &str) -> usize {
    if pattern.len() < 1 {
        return 1;
    }
    let mut res = 0;
    for i in 0..towels.len() {
        if pattern.starts_with(towels[i]) {
            res += amount_naive(towels, &pattern[towels[i].len()..]);
        }
    }
    res
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 16);
    }
}
