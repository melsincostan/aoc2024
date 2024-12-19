pub fn solve(path: &str) -> u32 {
    0
}

fn possible(towels: &Vec<&str>, pattern: &str) -> bool {
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
        assert_eq!(solve("sample.txt"), 0);
    }

    #[test]
    fn test_possible() {
        assert_eq!(possible(&vec!["rg", "r"], "rgrgr"), true);
        assert_eq!(possible(&vec!["rgr", "r", "rg"], "rgrgr"), true);
        assert_eq!(possible(&vec!["rgr", "grgr"], "rgrgr"), false);
    }
}
