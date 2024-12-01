use std::{cmp, fs};

pub fn solve(path: &str) -> u32 {
    let contents = fs::read_to_string(path).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    let (mut fl, mut sl) = parse_lines(lines);
    // need to compare lowest against lowest, etc...
    // easiest way to get there is to sort both!
    fl.sort_unstable();
    sl.sort_unstable();
    distances(&fl, &sl).into_iter().sum()
}

pub fn parse_lines(lines: Vec<&str>) -> (Vec<u32>, Vec<u32>) {
    let mut fl: Vec<u32> = vec![];
    let mut sl: Vec<u32> = vec![];
    lines.into_iter().for_each(|line: &str| {
        let res = parse_line(line);
        fl.push(res.0);
        sl.push(res.1);
    });
    (fl, sl)
}

fn parse_line(line: &str) -> (u32, u32) {
    let s = line.split("   ").collect::<Vec<&str>>();
    assert!(s.len() == 2); // check that there are two lists only...
    let flv = s[0]
        .parse::<u32>()
        .expect("could not parse first value to u32");
    let slv = s[1]
        .parse::<u32>()
        .expect("could not parse second list value into u32");
    (flv, slv)
}

fn distances(s1: &Vec<u32>, s2: &Vec<u32>) -> Vec<u32> {
    assert!(s1.len() == s2.len());
    let mut res: Vec<u32> = vec![];
    for i in 0..s1.len() {
        res.push(cmp::max(s1[i], s2[i]) - cmp::min(s1[i], s2[i]));
    }
    res
}

#[cfg(test)]
mod test {
    use crate::part1::{distances, parse_line, parse_lines, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 11);
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("1   2"), (1, 2));
        assert_eq!(parse_line("2   1"), (2, 1));
    }

    #[test]
    #[should_panic]
    fn test_parse_line_invalid() {
        parse_line("1   2   3");
        parse_line("1 2");
        parse_line("1  2");
        parse_line("1    2");
        parse_line("1\t2");
    }

    #[test]
    fn test_parse_lines() {
        // really not all that useful but better have it there for completions' sake :3
        let l_in: Vec<&str> = vec!["1   2", "2   3", "3   4"];
        let fl_out: Vec<u32> = vec![1, 2, 3];
        let sl_out: Vec<u32> = vec![2, 3, 4];
        assert_eq!(parse_lines(l_in), (fl_out, sl_out));
    }

    #[test]
    fn test_distances() {
        // not sorted, but the math is what is getting tested here!
        let l_in: Vec<u32> = vec![3, 4, 2, 1, 3, 3];
        let r_in: Vec<u32> = vec![4, 3, 5, 3, 9, 3];

        // explicit operations to be clearer about what should happen
        let expected: Vec<u32> = vec![4 - 3, 4 - 3, 5 - 2, 3 - 1, 9 - 3, 3 - 3];
        assert_eq!(distances(&l_in, &r_in), expected);
    }

    #[test]
    #[should_panic]
    fn test_distances_invalid() {
        // :neocat_explode:
        let l_in: Vec<u32> = vec![1, 2, 3, 4, 5];
        let r_in: Vec<u32> = vec![1, 2, 3, 4, 5, 6]; // oh no, longer!
        distances(&l_in, &r_in);
    }
}
