use std::{cmp, fs};

pub fn solve(path: &str) -> u32 {
    let contents = fs::read_to_string(path).unwrap();
    let lines = contents.lines().collect::<Vec<&str>>();
    let (mut fl, mut sl) = parse_lines(lines);
    fl.sort_unstable();
    sl.sort_unstable();
    distances(&fl, &sl).into_iter().sum()
}

fn parse_lines(lines: Vec<&str>) -> (Vec<u32>, Vec<u32>) {
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
