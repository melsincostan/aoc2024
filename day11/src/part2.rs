use std::{collections::HashMap, fs};

pub fn solve(path: &str) -> u64 {
    let mut cache: HashMap<(u64, u64), u64> = HashMap::new();
    fs::read_to_string(path)
        .unwrap()
        .split_whitespace()
        .map(|i| stone_amount(&mut cache, i.parse::<u64>().unwrap(), 75))
        .sum()
}

fn stone_amount(cache: &mut HashMap<(u64, u64), u64>, value: u64, iter: u64) -> u64 {
    if iter == 0 {
        1
    } else if cache.contains_key(&(value, iter)) {
        cache.get(&(value, iter)).unwrap().to_owned()
    } else if value == 0 {
        let res = stone_amount(cache, 1, iter - 1);
        cache.insert((value, iter), res);
        res
    } else if value.to_string().len() % 2 == 0 {
        let s = split(value.to_string());
        let res = stone_amount(cache, s.0, iter - 1) + stone_amount(cache, s.1, iter - 1);
        cache.insert((value, iter), res);
        res
    } else {
        let res = stone_amount(cache, value * 2024, iter - 1);
        cache.insert((value, iter), res);
        res
    }
}

fn split(value: String) -> (u64, u64) {
    let a = value[0..value.len() / 2].parse::<u64>().unwrap();
    let b = value[value.len() / 2..value.len()].parse::<u64>().unwrap();
    (a, b)
}
