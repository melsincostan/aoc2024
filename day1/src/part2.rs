use std::{collections::HashMap, fs};

use crate::part1;

pub fn solve(path: &str) -> u32 {
    let contents = fs::read_to_string(path).unwrap();
    let lines: Vec<&str> = contents.lines().collect();
    let (fl, sl) = part1::parse_lines(lines); // no need to reinvent the wheel...
    let num_map = count(sl);
    fl.into_iter()
        .map(|num: u32| similarity(&num_map, num))
        .sum()
}

fn count(l: Vec<u32>) -> HashMap<u32, u32> {
    let mut m: HashMap<u32, u32> = HashMap::new();
    l.into_iter().for_each(|num| {
        let curr_val = m.get(&num);
        if curr_val.is_some() {
            m.insert(num, curr_val.unwrap() + 1);
        } else {
            m.insert(num, 1);
        }
    });
    m
}

fn similarity(m: &HashMap<u32, u32>, num: u32) -> u32 {
    let mul = m.get(&num).unwrap_or(&0);
    num * mul
}

#[cfg(test)]
mod test {
    use crate::part2::{count, similarity, solve};
    use std::collections::HashMap;

    #[test]
    pub fn test_solve() {
        assert_eq!(solve("sample.txt"), 31)
    }

    #[test]
    pub fn test_count() {
        let input: Vec<u32> = vec![1, 2, 2, 3, 3, 3];
        let expect: HashMap<u32, u32> = HashMap::from([(1, 1), (2, 2), (3, 3)]);
        assert_eq!(count(input), expect);
    }

    #[test]
    pub fn test_similarity() {
        let m: HashMap<u32, u32> = HashMap::from([(1, 5), (2, 2), (3, 3)]);
        assert_eq!(similarity(&m, 3), 3 * 3);
        assert_eq!(similarity(&m, 2), 2 * 2);
        assert_eq!(similarity(&m, 1), 1 * 5);
        assert_eq!(similarity(&m, 5), 5 * 0);
    }
}
