use std::fs;

pub fn solve(path: &str) -> u32 {
    fs::read_to_string(path)
        .unwrap()
        .split_whitespace()
        .map(|i| stone_amount(i.parse::<u32>().unwrap(), 25))
        .sum()
}

fn stone_amount(value: u32, iter: u32) -> u32 {
    if iter == 0 {
        1
    } else if value == 0 {
        stone_amount(1, iter - 1)
    } else if value.to_string().len() % 2 == 0 {
        let s = split(value.to_string());
        stone_amount(s.0, iter - 1) + stone_amount(s.1, iter - 1)
    } else {
        stone_amount(value * 2024, iter - 1)
    }
}

fn split(value: String) -> (u32, u32) {
    let a = value[0..value.len() / 2].parse::<u32>().unwrap();
    let b = value[value.len() / 2..value.len()].parse::<u32>().unwrap();
    (a, b)
}

#[cfg(test)]
mod test {
    use crate::part1::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 55312);
    }
}
