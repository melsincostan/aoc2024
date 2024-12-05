use std::{collections::HashMap, fs};

pub fn solve(path: &str) -> u32 {
    let input = fs::read_to_string(path).unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();
    let rules = parse_rules(parts[0]);
    assert_eq!(parts.len(), 2);
    0
}

fn parse_rules(raw: &str) -> HashMap<u32, Vec<u32>> {
    let mut res: HashMap<u32, Vec<u32>> = HashMap::new();
    raw.split_whitespace().map(parse_rule).for_each(|r| {
        if res.contains_key(&r.0) {
            let mut curr = res.get(&r.0).unwrap().to_vec();
            curr.push(r.1);
            res.insert(r.0, curr);
        } else {
            res.insert(r.0, vec![r.1]);
        }
    });
    res
}

fn parse_rule(raw: &str) -> (u32, u32) {
    let spl: Vec<u32> = raw
        .split("|")
        .map(|l| l.trim().parse::<u32>().unwrap())
        .collect();
    assert_eq!(spl.len(), 2);
    (spl[0], spl[1])
}

#[cfg(test)]
mod test {
    use crate::part1::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 143);
    }
}
