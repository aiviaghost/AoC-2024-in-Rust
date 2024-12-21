use std::{collections::HashMap, fs};

fn count_ways(target_towel: &str, patterns: &Vec<&str>, dp: &mut HashMap<String, u64>) -> u64 {
    if target_towel.is_empty() {
        return 1;
    }

    if let Some(&b) = dp.get(target_towel) {
        return b;
    }

    let mut s = 0;
    for pattern in patterns {
        if target_towel.ends_with(pattern) {
            s += count_ways(target_towel.strip_suffix(pattern).unwrap(), patterns, dp)
        }
    }

    dp.insert(target_towel.to_string(), s);

    return s;
}

fn solve_1(input: Vec<String>) {
    let mut input = input.split(|line| line.is_empty());
    let patterns = input
        .next()
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .split(", ")
        .collect::<Vec<_>>();
    let target_towels: Vec<_> = input.next().unwrap().into();

    let mut dp = HashMap::new();
    let ans = target_towels
        .into_iter()
        .filter(|towel| count_ways(towel, &patterns, &mut dp) > 0)
        .count();

    println!("{ans}");
}

fn solve_2(input: Vec<String>) {
    let mut input = input.split(|line| line.is_empty());
    let patterns = input
        .next()
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .split(", ")
        .collect::<Vec<_>>();
    let target_towels: Vec<_> = input.next().unwrap().into();

    let mut dp = HashMap::new();
    let ans: u64 = target_towels
        .into_iter()
        .map(|towel| count_ways(&towel, &patterns, &mut dp))
        .sum();

    println!("{ans}");
}

fn main() {
    let input: Vec<String> = fs::read_to_string("input.txt")
        .unwrap()
        .split_terminator('\n')
        .map(|x| x.to_string())
        .collect();

    solve_1(input.clone());
    solve_2(input.clone());
}
