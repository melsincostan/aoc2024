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
                let vector = (freq[j].0 - freq[i].0, freq[j].1 - freq[i].1);
                let mut factor = 1;
                loop {
                    let pos = (freq[i].0 + factor * vector.0, freq[i].1 + factor * vector.1);
                    if pos.0 >= 0 && pos.0 < max_x && pos.1 >= 0 && pos.1 < max_y {
                        nodes.insert(pos);
                        factor += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        nodes
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
