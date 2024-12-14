use std::{
    collections::{HashMap, HashSet},
    fs, io,
};

pub fn solve(path: &str) -> i64 {
    let robots: Vec<((i64, i64), (i64, i64))> = fs::read_to_string(path)
        .unwrap()
        .lines()
        .map(|l| parse_robot(l))
        .collect();
    let mut i = 0;
    loop {
        i += 1;
        let pos: HashSet<(i64, i64)> = HashSet::from_iter(
            robots
                .iter()
                .map(|r| pos_in(i, r.0 .0, r.0 .1, r.1 .0, r.1 .1, 101, 103)),
        );
        for y in 0..103 {
            for x in 0..101 {
                if pos.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }

        let mut done = false;
        let mut lines: HashMap<i64, (i64, i64)> = HashMap::new();
        pos.iter().for_each(|p| {
            if lines.contains_key(&p.1) {
                let mut l = lines.get(&p.1).unwrap().to_owned();
                if p.0 == l.0 - 1 {
                    l.0 = p.0;
                } else if p.0 == l.1 + 1 {
                    l.1 = p.0;
                }

                lines.insert(p.1, l);

                if l.1 - l.0 >= 7 {
                    done = true;
                }
            } else {
                lines.insert(p.1, (p.0, p.0));
            }
        });
        if done {
            return i;
        } else {
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }
    }
}

pub fn pos_in(
    steps: i64,
    start_x: i64,
    start_y: i64,
    vel_x: i64,
    vel_y: i64,
    grid_x: i64,
    grid_y: i64,
) -> (i64, i64) {
    let mut x = (start_x + steps * vel_x) % grid_x;
    let mut y = (start_y + steps * vel_y) % grid_y;
    if x < 0 {
        x = grid_x - (x.abs() % grid_x);
    }

    if y < 0 {
        y = grid_y - (y.abs() % grid_y);
    }
    (x, y)
}

fn parse_robot(input: &str) -> ((i64, i64), (i64, i64)) {
    let raw_sets: Vec<&str> = input.split(" ").collect();
    assert_eq!(raw_sets.len(), 2);
    let start = parse_set(raw_sets[0]);
    let velocity = parse_set(raw_sets[1]);
    (start, velocity)
}

fn parse_set(input: &str) -> (i64, i64) {
    let raw_coords: Vec<&str> = input[2..input.len()].split(",").collect();
    assert_eq!(raw_coords.len(), 2);
    let x = raw_coords[0].parse::<i64>().unwrap();
    let y = raw_coords[1].parse::<i64>().unwrap();
    (x, y)
}
