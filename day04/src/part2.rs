use std::fs;

pub fn solve(path: &str) -> u32 {
    let grid: Vec<Vec<char>> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let mut total = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == 'A' {
                total += if is_x_mas(x, y, &grid) { 1 } else { 0 };
            }
        }
    }
    total
}

// assumes the caracter at [x, y] is an A!!
fn is_x_mas(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    if y < 1 || y + 1 >= grid.len() || x < 1 || x + 1 >= grid[y].len() {
        false
    } else {
        let a = (grid[y - 1][x - 1] == 'M' && grid[y + 1][x + 1] == 'S')
            || (grid[y - 1][x - 1] == 'S' && grid[y + 1][x + 1] == 'M');
        let b = (grid[y - 1][x + 1] == 'M' && grid[y + 1][x - 1] == 'S')
            || (grid[y - 1][x + 1] == 'S' && grid[y + 1][x - 1] == 'M');
        a && b
    }
}

#[cfg(test)]
mod test {
    use crate::part2::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 9)
    }
}
