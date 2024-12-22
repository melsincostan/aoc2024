use std::fs;

const LIM: u64 = 16777216;

pub fn solve(path: &str) -> u64 {
    let raw = fs::read_to_string(path).unwrap();
    let input = parse(raw);
    input
        .iter()
        .map(|num| {
            let mut s = *num;
            for _ in 0..2000 {
                next(&mut s);
            }
            s
        })
        .sum()
}

fn parse(input: String) -> Vec<u64> {
    input
        .lines()
        .map(|l| l.trim().parse::<u64>().unwrap())
        .collect()
}

fn next(s: &mut u64) {
    let mul64 = *s * 64;
    *s = (*s ^ mul64) % LIM;
    let div32 = *s / 32;
    *s = (*s ^ div32) % LIM;
    let mul2048 = *s * 2048;
    *s = (*s ^ mul2048) % LIM
}

#[cfg(test)]
mod test {

    use crate::part1::{next, parse, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 37327623);
    }

    #[test]
    fn test_next() {
        let mut a = 123;
        let mut b = 15887950;
        next(&mut a);
        next(&mut b);
        assert_eq!(a, 15887950);
        assert_eq!(b, 16495136);
    }

    #[test]
    fn test_parse() {
        let input = "1\n2\n3\n4".to_string();
        assert_eq!(parse(input), vec![1, 2, 3, 4]);
    }
}
