use std::{collections::HashSet, fs};

use crate::part1::parse_grid;

pub fn solve(path: &str) -> usize {
    let grid: Vec<Vec<char>> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let gr_y: i32 = grid.len().try_into().unwrap();
    let gr_x: i32 = grid[0].len().try_into().unwrap();
    let frequencies = parse_grid(&grid);
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    frequencies.into_values().for_each(|v| {
        let res = get_antinodes(&v, gr_x, gr_y);
        antinodes.extend(res);
    });
    antinodes.len()
}

fn get_antinodes(freq: &Vec<(i32, i32)>, max_x: i32, max_y: i32) -> HashSet<(i32, i32)> {
    if freq.len() <= 1 {
        HashSet::new()
    } else {
        let mut nodes: HashSet<(i32, i32)> = HashSet::new();

        for i in 0..freq.len() {
            for j in 0..freq.len() {
                if i == j {
                    continue;
                }
                // y = a*x + b
                let fun = get_line_func(freq[i], freq[j]);
                for k in 0..max_x {
                    let fy = fun(k);
                    if fy.fract() == 0.0 {
                        // if fy doesn't have any fractional part then it is an usable coordinate (maybe)
                        let y: i32 = fy.trunc() as i32;
                        if y >= 0 && y < max_y {
                            nodes.insert((k, y));
                        }
                    }
                }
            }
        }
        nodes
    }
}

fn get_line_func(pa: (i32, i32), pb: (i32, i32)) -> impl Fn(i32) -> f64 {
    let x1: f64 = pa.0.try_into().unwrap();
    let y1: f64 = pa.1.try_into().unwrap();
    let x2: f64 = pb.0.try_into().unwrap();
    let y2: f64 = pb.1.try_into().unwrap();
    let slope: f64 = (y2 - y1) / (x2 - x1);
    let intercept: f64 = y1 - slope * x1;
    move |x: i32| -> f64 {
        let fx: f64 = x.try_into().unwrap();
        slope * fx + intercept
    }
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 34);
    }
}
