use std::fs;

use crate::part1::{parse, run_code};

pub fn solve(path: &str) -> u32 {
    let raw = fs::read_to_string(path).unwrap();
    let prog = parse(raw);
    let mut ba = 0;
    let mut res: (Vec<u32>, u32, u32, u32);
    loop {
        res = run_code(&prog.0, ba, prog.2, prog.3);
        if res.0.len() == prog.0.len() && res.0 == prog.0 {
            break;
        }
        ba += 1;
    }
    ba
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 1);
    }
}
