use std::{cmp::Ordering, collections::HashMap, collections::HashSet, fs};

use crate::part1::{check_update, middle, parse_rules, parse_update};

pub fn solve(path: &str) -> u32 {
    let input = fs::read_to_string(path).unwrap();
    let parts: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(parts.len(), 2);
    let rules = parse_rules(parts[0]);
    let updates: Vec<Vec<u32>> = parts[1].split_whitespace().map(parse_update).collect();
    updates
        .into_iter()
        .filter(|u| !check_update(&rules, u))
        .map(|u| {
            let mut arr = u.to_owned();
            arr.sort_by(|a, b| compare_pos(&rules, a, b));
            arr
        })
        .map(middle)
        .sum()
}

fn compare_pos(rules: &HashMap<u32, HashSet<u32>>, a: &u32, b: &u32) -> Ordering {
    let r_a = rules.get(&a);
    let r_b = rules.get(&b);

    if r_a.is_some_and(|s| s.contains(&b)) {
        Ordering::Less
    } else if r_b.is_some_and(|r| r.contains(&a)) {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 123);
    }
}
