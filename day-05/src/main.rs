use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
};

enum Part {
    One,
    Two,
}

fn get_data(input: Vec<String>) -> Vec<(Part, u32)> {
    let mut data = input.split(|line| line.is_empty());
    let pairs = data
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            let mut ab = line.split(|c| c == '|');
            (
                ab.next().unwrap().parse::<u32>().unwrap(),
                ab.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let mut num_lines = data
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            line.split(|c| c == ',')
                .map(|c| c.parse::<u32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut follows = HashMap::<u32, HashSet<u32>>::new();
    for (a, b) in pairs {
        follows.entry(a).or_default().insert(b);
    }

    num_lines
        .iter_mut()
        .map(|nums| {
            let prev = nums.clone();
            nums.sort_by(|a, b| {
                if let Some(entry) = follows.get(a) {
                    if entry.contains(b) {
                        return Ordering::Less;
                    }
                }
                if let Some(entry) = follows.get(b) {
                    if entry.contains(a) {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            });

            (
                if prev == *nums { Part::One } else { Part::Two },
                nums[nums.len() / 2],
            )
        })
        .collect()
}

fn solve_1(input: Vec<String>) {
    let ans = get_data(input);

    let ans = ans
        .into_iter()
        .filter_map(|(part, page)| match part {
            Part::One => Some(page),
            Part::Two => None,
        })
        .sum::<u32>();

    println!("{ans}")
}

fn solve_2(input: Vec<String>) {
    let ans = get_data(input);

    let ans = ans
        .into_iter()
        .filter_map(|(part, page)| match part {
            Part::One => None,
            Part::Two => Some(page),
        })
        .sum::<u32>();

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
