use std::fs;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Void {
    start: usize,
    leftover_space: u8,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct File {
    start: usize,
    size: u8,
}

pub fn solve(path: &str) -> usize {
    let input = fs::read_to_string(path).unwrap();
    let trimmed = input.trim();
    let parsed = parse_input(&trimmed);
    let defragged = defrag(&parsed.0, &parsed.1);
    let formatted = format(&defragged, parsed.2);
    checksum(&formatted)
}

fn checksum(input: &Vec<String>) -> usize {
    let mut acc = 0;

    for i in 0..input.len() {
        if input[i] != "." {
            acc += i * input[i].to_string().parse::<usize>().unwrap();
        }
    }

    acc
}

fn format(f: &Vec<File>, size: usize) -> Vec<String> {
    let mut res: Vec<String> = ".".repeat(size).chars().map(|c| c.to_string()).collect();

    for i in 0..f.len() {
        for j in f[i].start..(f[i].start + f[i].size as usize) {
            res[j] = i.to_string();
        }
    }

    res
}

fn defrag(in_voids: &Vec<Void>, in_files: &Vec<File>) -> Vec<File> {
    let mut voids = in_voids.clone();
    let mut files = in_files.clone();
    for i in (0..files.len()).rev() {
        // in reverse order, try once for each file
        for j in 0..voids.len() {
            if voids[j].start > files[i].start {
                break;
            }
            if voids[j].leftover_space >= files[i].size {
                files[i].start = voids[j].start;
                voids[j].start = voids[j].start + files[i].size as usize;
                voids[j].leftover_space -= files[i].size;
            }
        }
    }
    files
}

fn parse_input(input: &str) -> (Vec<Void>, Vec<File>, usize) {
    let arr: Vec<char> = input.chars().collect();
    let mut voids: Vec<Void> = vec![];
    let mut files: Vec<File> = vec![];
    let mut pos = 0;
    for i in 0..arr.len() {
        let size = arr[i].to_string().parse::<u8>().unwrap();
        if i % 2 == 0 {
            files.push(File { start: pos, size });
        } else {
            voids.push(Void {
                start: pos,
                leftover_space: size,
            });
        }
        pos += size as usize;
    }
    (voids, files, pos)
}

#[cfg(test)]
mod test {
    use crate::part2::{checksum, format, parse_input, solve};

    use super::{File, Void};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), 2858);
    }

    #[test]
    fn test_parse_input() {
        let a = "12345";
        let av = vec![
            Void {
                start: 1,
                leftover_space: 2,
            },
            Void {
                start: 6,
                leftover_space: 4,
            },
        ];
        let af = vec![
            File { start: 0, size: 1 },
            File { start: 3, size: 3 },
            File { start: 10, size: 5 },
        ];
        assert_eq!(parse_input(a), (av, af, 15));
    }

    #[test]
    fn test_format() {
        let af = vec![
            File { start: 0, size: 1 },
            File { start: 3, size: 3 },
            File { start: 10, size: 5 },
        ];
        assert_eq!(
            format(&af, 15),
            "0..111....22222"
                .chars()
                .map(|c| c.to_string())
                .collect::<Vec<String>>()
        )
    }

    #[test]
    fn test_checksum() {
        let a: Vec<String> = "00992111777.44.333....5555.6666.....8888.."
            .chars()
            .map(|c| c.to_string())
            .collect();
        assert_eq!(checksum(&a), 2858);
    }
}
