use std::fs;

pub fn solve(input: &str) -> usize {
    let input = fs::read_to_string(input).unwrap();
    let trimmed = input.trim();
    let parsed = parse_input(&trimmed);
    let defragged = defrag(&parsed);
    checksum(&defragged)
}

// only run defragmented inputs through this!
fn checksum(input: &Vec<String>) -> usize {
    let mut acc = 0;

    for i in 0..input.len() {
        // first block is always going to be multiplicated by zero, theres no small savings :>
        if input[i] == "." {
            // reached the end of the blocks!
            break;
        }

        acc += i * input[i].to_string().parse::<usize>().unwrap();
    }

    acc
}

fn defrag(input: &Vec<String>) -> Vec<String> {
    let mut res = input.clone();
    'outside: for i in (0..res.len()).rev() {
        if res[i] != "." {
            // not empty space
            'inside: for j in 0..res.len() {
                if j >= i - 1 {
                    break 'outside;
                }

                if res[j] == "." {
                    res[j] = res[i].clone();
                    res[i] = ".".to_string();
                    break 'inside; // no point in going further, we're done
                }
            }
        }
    }
    res
}

fn parse_input(input: &str) -> Vec<String> {
    let parsed_nums: Vec<usize> = input
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect();
    let mut res: Vec<String> = vec![];
    for i in 0..parsed_nums.len() {
        if i % 2 == 0 {
            // even, represents a file
            let id = i / 2;
            for _ in 0..parsed_nums[i] {
                res.push(format!("{}", id).clone());
            }
        } else {
            // odd, represents whitespace
            let mut entries: Vec<String> = "."
                .repeat(parsed_nums[i])
                .chars()
                .map(|c| c.to_string())
                .collect();

            res.append(&mut entries);
        }
    }
    res
}

#[cfg(test)]
mod test {

    use crate::part1::{checksum, defrag, parse_input, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 1928);
    }

    #[test]
    fn test_parse_input() {
        assert_eq!(
            parse_input("12345"),
            "0..111....22222"
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        );
        assert_eq!(
            parse_input("2333133121414131402"),
            "00...111...2...333.44.5555.6666.777.888899"
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        )
    }

    #[test]
    fn test_defrag() {
        let a: Vec<String> = "0..111....22222".chars().map(|c| c.to_string()).collect();
        let b: Vec<String> = "00...111...2...333.44.5555.6666.777.888899"
            .chars()
            .map(|c| c.to_string())
            .collect();
        assert_eq!(
            defrag(&a),
            "022111222......"
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        );
        assert_eq!(
            defrag(&b),
            "0099811188827773336446555566.............."
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        );
    }

    #[test]
    fn test_checksum() {
        let a = "0099811188827773336446555566.............."
            .chars()
            .map(|c| c.to_string())
            .collect::<Vec<String>>();
        assert_eq!(checksum(&a), 1928);
    }
}
