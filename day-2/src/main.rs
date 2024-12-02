use std::{fs, iter::repeat};

fn solve_1(input: Vec<String>) {
    let ans: i32 = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .flat_map(|nums| [nums.clone(), nums.into_iter().rev().collect()])
        .map(|nums| {
            nums.iter()
                .zip(nums.iter().skip(1))
                .map(|(i, j)| i < j && (i - j).abs() <= 3)
                .all(|p| p) as i32
        })
        .sum();

    println!("{ans}")
}

fn solve_2(input: Vec<String>) {
    let ans: i32 = input
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .flat_map(|nums| [nums.clone(), nums.into_iter().rev().collect()])
        .map(|nums| {
            repeat(nums.clone())
                .take(nums.len())
                .enumerate()
                .map(|(filter_idx, nums)| {
                    nums.into_iter()
                        .enumerate()
                        .filter_map(|(i, x)| if i != filter_idx { Some(x) } else { None })
                        .collect::<Vec<_>>()
                })
                .map(|nums| {
                    nums.iter()
                        .zip(nums.iter().skip(1))
                        .map(|(i, j)| i < j && (i - j).abs() <= 3)
                        .all(|p| p)
                })
                .any(|p| p) as i32
        })
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
