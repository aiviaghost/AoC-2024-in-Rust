use std::{
    collections::{HashMap, VecDeque},
    fs, usize,
};

fn solve_1(input: Vec<String>) {
    let input = (input[0].clone() + "0")
        .chars()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut seq = HashMap::<u64, u64>::new();
    let mut active_positions = Vec::<u64>::new();
    let mut inactive_positions = VecDeque::<u64>::new();

    let mut curr_active_pos: u64 = 0;
    let mut curr_id: u64 = 0;
    for (&file_size, &empty_space) in input.iter().step_by(2).zip(input.iter().skip(1).step_by(2)) {
        for _ in 0..file_size {
            active_positions.push(curr_active_pos);
            seq.insert(curr_active_pos, curr_id);
            curr_active_pos += 1;
        }

        for _ in 0..empty_space {
            inactive_positions.push_back(curr_active_pos);
            curr_active_pos += 1;
        }

        curr_id += 1;
    }

    while !inactive_positions.is_empty() {
        let new_pos = inactive_positions.pop_front().unwrap();
        let active_pos = active_positions.pop().unwrap();
        if active_pos < new_pos {
            break;
        }
        let id = seq.remove(&active_pos).unwrap();
        seq.insert(new_pos, id);
    }

    let ans: u64 = seq.iter().map(|(pos, v)| pos * v).sum();

    println!("{ans}");
}

fn solve_2(input: Vec<String>) {
    let input = (input[0].clone() + "0")
        .chars()
        .map(|c| c.to_string().parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut active_positions = Vec::<(u64, u64, u64)>::new();
    let mut inactive_positions = Vec::<(u64, u64)>::new();

    let mut curr_active_pos: u64 = 0;
    let mut curr_id: u64 = 0;
    for (&file_size, &empty_space) in input.iter().step_by(2).zip(input.iter().skip(1).step_by(2)) {
        active_positions.push((curr_active_pos, file_size, curr_id));
        curr_active_pos += file_size;

        inactive_positions.push((curr_active_pos, empty_space));
        curr_active_pos += empty_space;

        curr_id += 1;
    }

    let mut final_positions = Vec::<(u64, u64, u64)>::new();
    while !active_positions.is_empty() {
        let (pos, file_size, id) = active_positions.pop().unwrap();

        let mut to_remove = usize::MAX;
        for (i, &(inactive_pos, available_space)) in inactive_positions.iter().enumerate() {
            if inactive_pos < pos && available_space >= file_size {
                to_remove = i;
                break;
            }
        }

        if to_remove < usize::MAX {
            let (inactive_pos, available_space) = inactive_positions.remove(to_remove);
            inactive_positions.push((inactive_pos + file_size, available_space - file_size));
            final_positions.push((inactive_pos, file_size, id));
        } else {
            final_positions.push((pos, file_size, id));
        }

        inactive_positions.sort();
    }

    let ans: u64 = final_positions
        .iter()
        .map(|(pos, file_size, id)| id * (2 * pos + file_size - 1) * file_size / 2)
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
