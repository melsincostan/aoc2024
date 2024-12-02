use std::fs;

use crate::part1;

pub fn solve(path: &str) -> usize {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(part1::parse_line)
        .filter(check_line)
        .count()
}

fn check_line(line: &Vec<i32>) -> bool {
    check_line_int(line, false)
}

fn check_line_int(line: &Vec<i32>, recursing: bool) -> bool {
    assert!(line.len() >= 2);

    let asc = line[0] < line[1];

    for i in 0..(line.len() - 1) {
        if !ok(asc, line[i], line[i + 1]) {
            if recursing {
                return false;
            }
            let mut wf = line.to_owned();
            let mut ws = line.to_owned();
            let mut wz = line.to_owned();
            wf.remove(i);
            ws.remove(i + 1);
            wz.remove(0);

            if check_line_int(&wf, true) || check_line_int(&ws, true) || check_line_int(&wz, true) {
            } else {
                println!("giving up on line at index {}", i);
                return brute(line);
            }
        }
    }
    true
}

fn brute(line: &Vec<i32>) -> bool {
    for i in 0..line.len() {
        let mut wi = line.to_owned();
        wi.remove(i);
        if check_line_int(&wi, true) {
            println!("recovered line by removing index {} (line data follows)", i);
            println!("\t{:?}", line);
            return true;
        }
    }
    false
}

fn ok(asc: bool, a: i32, b: i32) -> bool {
    let diff = a - b;
    if (asc && b <= a) || (!asc && b >= a) {
        return false;
    }

    if a == b || diff.abs() > 3 {
        return false;
    }

    true
}

#[cfg(test)]
mod test {
    use crate::part2::{check_line, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 4);
    }

    #[test]
    fn test_check_line() {
        assert_eq!(check_line(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(check_line(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(check_line(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(check_line(&vec![1, 3, 2, 4, 5]), true);
        assert_eq!(check_line(&vec![8, 6, 4, 4, 1]), true);
        assert_eq!(check_line(&vec![1, 3, 6, 7, 9]), true);
        assert_eq!(check_line(&vec![3, 1, 2, 4, 5]), true);
        assert_eq!(check_line(&vec![9, 10, 8, 7, 6]), true);
        assert_eq!(check_line(&vec![1, 12, 4, 3, 2]), false);
        assert_eq!(check_line(&vec![1, 2, 3, 4, 5, 2]), true);
    }
}
