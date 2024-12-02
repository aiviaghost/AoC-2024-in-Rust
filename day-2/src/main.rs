use std::{fs, iter::repeat};

fn solve_1(input: Vec<String>) {
    let ans: i32 = input
        .iter()
        .flat_map(|line| {
            let data: Vec<_> = line
                .split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            [data.clone(), data.into_iter().rev().collect()]
        })
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
        .flat_map(|data| [data.clone(), data.into_iter().rev().collect()])
        .map(|data| {
            repeat(data.clone())
                .take(data.len())
                .enumerate()
                .map(|(filter_idx, nums)| {
                    nums.into_iter()
                        .enumerate()
                        .filter_map(|(i, x)| if i != filter_idx { Some(x) } else { None })
                        .collect()
                })
                .map(|nums: Vec<_>| {
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
