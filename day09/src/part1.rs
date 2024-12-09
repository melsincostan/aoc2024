pub fn solve(input: &str) -> u32 {
    0
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

    use crate::part1::{parse_input, solve};

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
}
