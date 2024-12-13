pub fn solve(path: &str) -> u64 {
    1
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

#[cfg(test)]
mod test {
    use crate::part2::{bin_gcd, solve};

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
}
