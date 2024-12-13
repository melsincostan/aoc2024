use std::fs;

pub fn solve(path: &str) -> i64 {
    let file = fs::read_to_string(path).unwrap();
    let input = file.split("\n\n"); // get single games
    input
        .into_iter()
        .map(parse_game)
        .map(|g| {
            let res = result(g.0 .0, g.0 .1, g.1 .0, g.1 .1, g.2 .0, g.2 .1);
            if res.is_some() {
                let u = res.unwrap();
                u.0 * 3 + u.1
            } else {
                0
            }
        })
        .sum()
}

fn parse_game(input: &str) -> ((i64, i64), (i64, i64), (i64, i64)) {
    let lines: Vec<&str> = input.trim().split("\n").collect();
    assert_eq!(lines.len(), 3);
    let a = parse_coords(lines[0]);
    let b = parse_coords(lines[1]);
    let p = parse_coords(lines[2]);
    (a, b, p)
}

fn parse_coords(input: &str) -> (i64, i64) {
    let s: Vec<&str> = input.split(": ").collect();
    assert_eq!(s.len(), 2);
    let raw_coords: Vec<&str> = s[1].split(", ").collect();
    assert_eq!(raw_coords.len(), 2);
    (
        raw_coords[0][2..raw_coords[0].len()]
            .parse::<i64>()
            .unwrap(),
        raw_coords[1][2..raw_coords[1].len()]
            .parse::<i64>()
            .unwrap(),
    )
}

fn result(ax: i64, ay: i64, bx: i64, by: i64, px: i64, py: i64) -> Option<(i64, i64)> {
    // get the first result
    let top = px * by - py * bx;
    let bottom = ax * by - ay * bx;
    if bottom == 0 || top % bottom != 0 {
        // either we divide by 0 (apparently "a big no-no", or we push a button a non-integer amount (probably also a crime))
        None
    } else {
        let pa = (px * by - py * bx) / (ax * by - ay * bx); // get the amount of presses on a using some simplified form of the two equations
        let btop = pa * ax - px;
        if bx == 0 || btop % bx != 0 {
            // again, either division by 0 or non-integer result. Not super interesting...
            None
        } else {
            let pb = -(pa * ax - px) / bx; // now that we have that, we can use the x computation to get the amounts of presses on b
            Some((pa, pb))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{part1::result, part1::solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 480);
    }

    #[test]
    fn test_result() {
        assert_eq!(result(94, 34, 22, 67, 8400, 5400), Some((80, 40)));
        assert_eq!(result(26, 66, 67, 21, 12748, 12176), None);
    }
}
