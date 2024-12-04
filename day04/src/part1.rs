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
            if grid[y][x] == 'X' {
                total += if search_right(x, &grid[y]) { 1 } else { 0 };
                total += if search_left(x, &grid[y]) { 1 } else { 0 };
                total += if search_up(x, y, &grid) { 1 } else { 0 };
                total += if search_down(x, y, &grid) { 1 } else { 0 };
                total += if search_diag_tr(x, y, &grid) { 1 } else { 0 };
                total += if search_diag_tl(x, y, &grid) { 1 } else { 0 };
                total += if search_diag_br(x, y, &grid) { 1 } else { 0 };
                total += if search_diag_bl(x, y, &grid) { 1 } else { 0 };
            }
        }
    }
    total
}

fn search_right(x: usize, line: &Vec<char>) -> bool {
    if x + 3 >= line.len() {
        false
    } else {
        line[x + 1] == 'M' && line[x + 2] == 'A' && line[x + 3] == 'S'
    }
}

fn search_left(x: usize, line: &Vec<char>) -> bool {
    if x < 3 {
        false
    } else {
        line[x - 1] == 'M' && line[x - 2] == 'A' && line[x - 3] == 'S'
    }
}

fn search_up(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    if y < 3 {
        false
    } else {
        grid[y - 1][x] == 'M' && grid[y - 2][x] == 'A' && grid[y - 3][x] == 'S'
    }
}

fn search_down(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    if y + 3 >= grid.len() {
        false
    } else {
        grid[y + 1][x] == 'M' && grid[y + 2][x] == 'A' && grid[y + 3][x] == 'S'
    }
}

fn search_diag_tr(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    if (y < 3) || (x + 3 >= grid[y].len()) {
        false
    } else {
        grid[y - 1][x + 1] == 'M' && grid[y - 2][x + 2] == 'A' && grid[y - 3][x + 3] == 'S'
    }
}

fn search_diag_tl(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    if (y < 3) || (x < 3) {
        false
    } else {
        grid[y - 1][x - 1] == 'M' && grid[y - 2][x - 2] == 'A' && grid[y - 3][x - 3] == 'S'
    }
}

fn search_diag_br(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    if y + 3 >= grid.len() || x + 3 >= grid[y].len() {
        false
    } else {
        grid[y + 1][x + 1] == 'M' && grid[y + 2][x + 2] == 'A' && grid[y + 3][x + 3] == 'S'
    }
}

fn search_diag_bl(x: usize, y: usize, grid: &Vec<Vec<char>>) -> bool {
    if y + 3 >= grid.len() || x < 3 {
        false
    } else {
        grid[y + 1][x - 1] == 'M' && grid[y + 2][x - 2] == 'A' && grid[y + 3][x - 3] == 'S'
    }
}

#[cfg(test)]
mod test {
    use crate::part1::solve;

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 18)
    }
}
