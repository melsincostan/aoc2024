use std::{
    collections::{HashMap, HashSet},
    fs,
};

pub fn solve(path: &str) -> u32 {
    let input = fs::read_to_string(path).unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(parts.len(), 2);
    let rules = parse_rules(parts[0]);
    let updates: Vec<Vec<u32>> = parts[1].split_whitespace().map(parse_update).collect();
    updates
        .into_iter()
        .filter(|u| check_update(&rules, u))
        .map(middle)
        .sum()
}

pub fn middle(arr: Vec<u32>) -> u32 {
    assert_eq!(arr.len() % 2, 1); // how would one get the middle of an even list??
    arr[arr.len().div_ceil(2) - 1] // for some reason .div_floor() makes use of something unstable?
}

pub fn check_update(rules: &HashMap<u32, HashSet<u32>>, update: &Vec<u32>) -> bool {
    for i in 0..update.len() {
        let after: HashSet<u32> =
            HashSet::from_iter(update[i..update.len()].to_owned().into_iter());
        if rules.contains_key(&update[i]) {
            let u = after.intersection(rules.get(&update[i]).unwrap());
            let overlap = u.count();
            if overlap > 0 {
                return false;
            }
        } else {
            continue;
        }
    }
    true
}

pub fn parse_update(raw: &str) -> Vec<u32> {
    raw.trim()
        .split(",")
        .map(|s| s.parse::<u32>().unwrap())
        .collect()
}

pub fn parse_rules(raw: &str) -> HashMap<u32, HashSet<u32>> {
    // for each found entry, list what numbers must come before it
    let mut res: HashMap<u32, HashSet<u32>> = HashMap::new();
    raw.split_whitespace().map(parse_rule).for_each(|r| {
        if res.contains_key(&r.1) {
            // if a comes before b, then b must come after a
            let mut curr = res.get(&r.1).unwrap().to_owned();
            curr.insert(r.0);
            res.insert(r.1, curr);
        } else {
            res.insert(r.1, HashSet::from_iter(vec![r.0].into_iter()));
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
    (spl[0], spl[1]) // a must come before b
}

#[cfg(test)]
mod test {
    use crate::part1::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 143);
    }
}
