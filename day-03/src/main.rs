use std::fs;

use regex::Regex;

fn solve_1(input: Vec<String>) {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let ans: i32 = input
        .into_iter()
        .map(|line| {
            pattern
                .captures_iter(&line)
                .map(|c| c.extract())
                .map(|(_, [d1, d2])| d1.parse::<i32>().unwrap() * d2.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .sum();

    println!("{ans}")
}

fn solve_2(input: Vec<String>) {
    let pattern = Regex::new(r"(mul\((\d+),(\d+)\))|(do\(\))|(don't\(\))").unwrap();

    let ans: u64 = input
        .into_iter()
        .scan(true, |state, line| {
            Some(
                pattern
                    .captures_iter(&line)
                    .map(|c| match c.get(0).unwrap().as_str() {
                        "do()" => {
                            *state = true;
                            0
                        }
                        "don't()" => {
                            *state = false;
                            0
                        }
                        _ => {
                            let d1 = c.get(2).unwrap().as_str().parse::<u64>().unwrap();
                            let d2 = c.get(3).unwrap().as_str().parse::<u64>().unwrap();
                            if *state {
                                d1 * d2
                            } else {
                                0
                            }
                        }
                    })
                    .sum::<u64>(),
            )
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
