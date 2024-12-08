use std::{
    collections::{HashMap, HashSet},
    fs,
};

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
                let antinode = (freq[i].0 + 2 * vector.0, freq[i].1 + 2 * vector.1);
                if antinode.0 < max_x && antinode.1 < max_y && antinode.0 >= 0 && antinode.1 >= 0 {
                    nodes.insert(antinode);
                }
            }
        }
        nodes
    }
}

fn parse_grid(grid: &Vec<Vec<char>>) -> HashMap<char, Vec<(i32, i32)>> {
    let mut frequencies: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for y in 0..grid.len() {
        for x in 0..grid.len() {
            if grid[y][x].is_alphanumeric() {
                if frequencies.contains_key(&grid[y][x]) {
                    let mut old = frequencies.get(&grid[y][x]).unwrap().to_owned();
                    old.push((x.try_into().unwrap(), y.try_into().unwrap()));
                    frequencies.insert(grid[y][x], old);
                } else {
                    frequencies.insert(
                        grid[y][x],
                        vec![(x.try_into().unwrap(), y.try_into().unwrap())],
                    );
                }
            }
        }
    }
    frequencies
}

#[cfg(test)]
mod test {
    use crate::part1::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 14);
    }
}
