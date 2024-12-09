use std::fs;

pub fn solve(input: &str) -> usize {
    let input = fs::read_to_string(input).unwrap();
    let trimmed = input.trim();
    let parsed = parse_input(&trimmed);
    let defragged = defrag(&parsed);
    checksum(&defragged)
}

// only run defragmented inputs through this!
fn checksum(input: &Vec<char>) -> usize {
    let mut acc = 0;

    for i in 1..input.len() {
        // first block is always going to be multiplicated by zero, theres no small savings :>
        if input[i] == '.' {
            // reached the end of the blocks!
            break;
        }

        acc += i * input[i].to_string().parse::<usize>().unwrap();
    }

    acc
}

fn defrag(input: &Vec<char>) -> Vec<char> {
    let mut res = input.clone();
    'outside: for i in (0..res.len()).rev() {
        println!("{}", i);
        if res[i] != '.' {
            // not empty space
            'inside: for j in 0..res.len() {
                if j >= i - 1 {
                    // won't be possible to fit anymore, save a few cycles
                    break 'outside;
                }

                if res[j] == '.' {
                    res[j] = res[i];
                    res[i] = '.';
                    break 'inside; // no point in going further, we're done
                }
            }
        }
    }
    res
}

fn parse_input(input: &str) -> Vec<char> {
    let parsed_nums: Vec<usize> = input
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .collect();
    let mut res: Vec<char> = vec![];
    for i in 0..parsed_nums.len() {
        if i % 2 == 0 {
            // even, represents a file
            let id = i / 2;
            let mut entries: Vec<char> = id.to_string().repeat(parsed_nums[i]).chars().collect();
            res.append(&mut entries);
        } else {
            // odd, represents whitespace
            let mut entries: Vec<char> = ".".repeat(parsed_nums[i]).chars().collect();
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
            "0..111....22222".chars().collect::<Vec<char>>()
        );
        assert_eq!(
            parse_input("2333133121414131402"),
            "00...111...2...333.44.5555.6666.777.888899"
                .chars()
                .collect::<Vec<char>>()
        )
    }

    #[test]
    fn test_defrag() {
        let a: Vec<char> = "0..111....22222".chars().collect();
        let b: Vec<char> = "00...111...2...333.44.5555.6666.777.888899"
            .chars()
            .collect();
        assert_eq!(defrag(&a), "022111222......".chars().collect::<Vec<char>>());
        assert_eq!(
            defrag(&b),
            "0099811188827773336446555566.............."
                .chars()
                .collect::<Vec<char>>()
        );
    }

    #[test]
    fn test_checksum() {
        let a = "0099811188827773336446555566.............."
            .chars()
            .collect::<Vec<char>>();
        assert_eq!(checksum(&a), 1928);
    }
}
