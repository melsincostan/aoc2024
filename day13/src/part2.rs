pub fn solve(path: &str) -> u64 {
    1
}

pub fn has_solution(a: u32, b: u32, c: u32) -> bool {
    c % bin_gcd(a, b) == 0
}

pub fn bin_gcd(a: u32, b: u32) -> u32 {
    if a == b {
        a
    } else if a == 0 {
        b
    } else if b == 0 {
        b
    } else if a % 2 == 0 && b % 2 == 0 {
        2 * bin_gcd(a / 2, b / 2)
    } else if a % 2 == 0 {
        bin_gcd(a / 2, b)
    } else if b % 2 == 0 {
        bin_gcd(a, b / 2)
    } else if a <= b {
        bin_gcd(a, b - a)
    } else {
        bin_gcd(a - b, b)
    }
}

pub fn extended_euclid_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let r = extended_euclid_gcd(b % a, a);
        let x = r.2 - (b / a) * r.1;
        let y = r.1;
        (r.0, x, y)
    }
}

#[cfg(test)]
mod test {

    use crate::part2::{bin_gcd, extended_euclid_gcd, has_solution, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 1);
    }

    #[test]
    fn test_bin_gcd() {
        assert_eq!(bin_gcd(8, 12), 4);
        assert_eq!(bin_gcd(15, 9), 3);
        assert_eq!(bin_gcd(99938, 1), 1);
        assert_eq!(bin_gcd(30, 15), 15);
    }

    #[test]
    fn test_extendded_euclid_gcd() {
        assert_eq!(extended_euclid_gcd(55, 79), (1, 23, -16))
    }

    #[test]
    fn test_has_solution() {
        assert_eq!(has_solution(94, 22, 8400), true);
        assert_eq!(has_solution(34, 67, 5400), true);
        assert_eq!(has_solution(26, 67, 12748), true);
        assert_eq!(has_solution(66, 21, 12176), false);
    }
}
