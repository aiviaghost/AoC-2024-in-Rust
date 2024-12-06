use std::{collections::HashMap, fs};

fn get_lr_tuples(input: Vec<String>) -> Vec<(i32, i32)> {
    input
        .iter()
        .map(|line| {
            let mut lr = line.split_whitespace();
            (
                lr.next().unwrap().parse::<i32>().unwrap(),
                lr.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .collect()
}

fn solve_1(input: Vec<String>) {
    let (mut left, mut right): (Vec<_>, Vec<_>) = get_lr_tuples(input).into_iter().unzip();

    left.sort();
    right.sort();

    let ans: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("{ans}")
}

fn solve_2(input: Vec<String>) {
    let mut left = HashMap::<i32, i32>::new();
    let mut right = HashMap::<i32, i32>::new();
    for (l, r) in get_lr_tuples(input) {
        *left.entry(l).or_default() += 1;
        *right.entry(r).or_default() += 1;
    }

    let ans: i32 = left
        .iter()
        .map(|(k, v)| k * v * right.get(k).or(Some(&0)).unwrap())
        .sum();

    println!("{ans}")
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
