use core::panic;
use std::fs;

pub fn solve(path: &str) -> String {
    let raw = fs::read_to_string(path).unwrap();
    let prog = parse(raw);
    let res = run_code(&prog.0, prog.1, prog.2, prog.3);
    let out: Vec<String> = res.0.iter().map(|n| format!("{}", n)).collect();
    out.join(",")
}

pub fn parse(input: String) -> (Vec<u32>, u32, u32, u32) {
    let spl: Vec<&str> = input.lines().collect();
    assert_eq!(spl.len(), 5);
    let a = parse_reg(spl[0]);
    let b = parse_reg(spl[1]);
    let c = parse_reg(spl[2]);
    let program = parse_instructions(spl[4]);
    (program, a, b, c)
}

fn parse_instructions(input: &str) -> Vec<u32> {
    let spl: Vec<&str> = input.split(": ").collect();
    assert_eq!(spl.len(), 2);
    spl[1]
        .split(",")
        .map(|e| e.parse::<u32>().unwrap())
        .collect()
}

fn parse_reg(input: &str) -> u32 {
    let spl: Vec<&str> = input.split(": ").collect();
    assert_eq!(spl.len(), 2);
    spl[1].parse::<u32>().unwrap()
}

pub fn run_code(
    instructions: &Vec<u32>,
    start_a: u32,
    start_b: u32,
    start_c: u32,
) -> (Vec<u32>, u32, u32, u32) {
    let mut a = start_a;
    let mut b = start_b;
    let mut c = start_c;
    let mut ptr = 0;
    let mut out: Vec<u32> = vec![];
    while ptr < instructions.len() {
        exec(
            instructions[ptr],
            instructions[ptr + 1],
            &mut a,
            &mut b,
            &mut c,
            &mut ptr,
            &mut out,
        );
    }
    (out, a, b, c)
}

fn exec(
    opcode: u32,
    operand: u32,
    a: &mut u32,
    b: &mut u32,
    c: &mut u32,
    ptr: &mut usize,
    out: &mut Vec<u32>,
) {
    match opcode {
        0 => {
            // ADV
            // a power of two is just a right shift of 1 since binary is powers of 2
            // working on ints is nice
            let res = *a / (1 << val(operand, a, b, c));
            *a = res;
            *ptr += 2;
        }
        1 => {
            // BXL
            let res = *b ^ operand;
            *b = res;
            *ptr += 2;
        }
        2 => {
            // BST
            // premature optimisation: we only keep the lowest 3 bits so use a bitwise operator :3
            let res = val(operand, a, b, c) & 0b00000111;
            *b = res;
            *ptr += 2;
        }
        3 => {
            // JNZ
            if *a != 0 {
                *ptr = operand as usize;
            } else {
                *ptr += 2;
            }
        }
        4 => {
            // BXC
            let res = *b ^ *c;
            *b = res;
            *ptr += 2;
        }
        5 => {
            // OUT
            let res = val(operand, a, b, c) & 0b00000111;
            out.push(res);
            *ptr += 2;
        }
        6 => {
            // BDV
            let res = *a / (1 << val(operand, a, b, c));
            *b = res;
            *ptr += 2;
        }
        7 => {
            // CDV
            let res = *a / (1 << val(operand, a, b, c));
            *c = res;
            *ptr += 2;
        }
        _ => panic!("unknown instruction"),
    }
}

fn val(operand: u32, a: &u32, b: &u32, c: &u32) -> u32 {
    match operand {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => *a,
        5 => *b,
        6 => *c,
        7 => panic!("reserved operand. should not appear"),
        _ => panic!("unknown operand"),
    }
}

#[cfg(test)]
mod test {
    use crate::part1::{run_code, solve};

    #[test]
    fn test_solve() {
        assert_eq!(solve("sample.txt"), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_run_code() {
        assert_eq!(run_code(vec![2, 6], 0, 0, 9), (vec![], 0, 1, 9));
        assert_eq!(
            run_code(vec![5, 0, 5, 1, 5, 4], 10, 0, 0),
            (vec![0, 1, 2], 10, 0, 0)
        );
        assert_eq!(
            run_code(vec![0, 1, 5, 4, 3, 0], 2024, 0, 0),
            (vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0], 0, 0, 0)
        );
        assert_eq!(run_code(vec![1, 7], 0, 29, 0), (vec![], 0, 26, 0));
        assert_eq!(
            run_code(vec![4, 0], 0, 2024, 43690),
            (vec![], 0, 44354, 43690)
        );
    }
}
