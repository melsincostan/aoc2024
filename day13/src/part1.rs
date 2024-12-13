pub fn solve(path: &str) -> i64 {
    0
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
    use crate::{part1::solve, part2::result};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 0);
    }

    #[test]
    fn test_result() {
        assert_eq!(result(94, 34, 22, 67, 8400, 5400), Some((80, 40)));
        assert_eq!(result(26, 66, 67, 21, 12748, 12176), None);
    }
}
