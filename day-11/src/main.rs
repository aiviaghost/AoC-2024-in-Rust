use std::{collections::HashMap, fs};

fn solve(x: u64, curr_depth: u64, target_depth: u64, dp: &mut HashMap<(u64, u64), u64>) -> u64 {
    if dp.contains_key(&(x, curr_depth)) {
        return dp[&(x, curr_depth)];
    }

    if curr_depth == target_depth {
        return 1;
    }

    let mut res = 0;

    if x == 0 {
        res = solve(1, curr_depth + 1, target_depth, dp);
    } else if x.to_string().len() % 2 == 0 {
        let x = x.to_string();
        let (left, right) = x.split_at(x.len() / 2);
        let (left, right): (u64, u64) = (left.parse().unwrap(), right.parse().unwrap());

        res += solve(left, curr_depth + 1, target_depth, dp);
        res += solve(right, curr_depth + 1, target_depth, dp);
    } else {
        res = solve(2024 * x, curr_depth + 1, target_depth, dp);
    }

    dp.insert((x, curr_depth), res);

    res
}

fn solve_1(input: Vec<u64>) {
    let mut dp = HashMap::<(u64, u64), u64>::new();

    let ans: u64 = input.iter().map(|&x| solve(x, 0, 25, &mut dp)).sum();

    println!("{ans}")
}

fn solve_2(input: Vec<u64>) {
    let mut dp = HashMap::<(u64, u64), u64>::new();

    let ans: u64 = input.iter().map(|&x| solve(x, 0, 75, &mut dp)).sum();

    println!("{ans}")
}

fn main() {
    let input: Vec<u64> = fs::read_to_string("input.txt")
        .unwrap()
        .split_terminator('\n')
        .next()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    solve_1(input.clone());
    solve_2(input.clone());
}
