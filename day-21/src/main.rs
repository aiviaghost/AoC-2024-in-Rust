use std::{collections::HashMap, fs};

const GAP: char = ' ';

fn create_lookups() -> (HashMap<char, (usize, usize)>, HashMap<char, (usize, usize)>) {
    let mut keypad_lookup = HashMap::new();
    let keypad: [[char; 3]; 4] = [
        ['7', '8', '9'],
        ['4', '5', '6'],
        ['1', '2', '3'],
        [GAP, '0', 'A'],
    ];
    for i in 0..4 {
        for j in 0..3 {
            keypad_lookup.insert(keypad[i][j], (i, j));
        }
    }

    let mut dirpad_lookup = HashMap::new();
    let dirpad: [[char; 3]; 2] = [[GAP, '^', 'A'], ['<', 'v', '>']];
    for i in 0..2 {
        for j in 0..3 {
            dirpad_lookup.insert(dirpad[i][j], (i, j));
        }
    }

    (keypad_lookup, dirpad_lookup)
}

fn shortest_path_button_to_button(
    button_start: char,
    button_end: char,
    lookup: &HashMap<char, (usize, usize)>,
) -> String {
    let (r1, c1) = lookup[&button_start];
    let (r2, c2) = lookup[&button_end];

    let vertical_moves = if r1 < r2 {
        "v".repeat(r2 - r1)
    } else {
        "^".repeat(r1 - r2)
    }
    .to_string();

    let going_right = c1 < c2;
    let horizontal_moves = if going_right {
        ">".repeat(c2 - c1)
    } else {
        "<".repeat(c1 - c2)
    }
    .to_string();

    let gap_position = lookup[&GAP];

    return if (going_right && (r2, c1) == gap_position)
        || (!going_right && (r1, c2) != gap_position)
    {
        horizontal_moves + &vertical_moves
    } else {
        vertical_moves + &horizontal_moves
    };
}

fn get_optimal_sequence(
    target_sequence: &Vec<char>,
    lookup: &HashMap<char, (usize, usize)>,
) -> Vec<Vec<char>> {
    target_sequence
        .iter()
        .scan('A', |prev_button, target_button| {
            let seq = shortest_path_button_to_button(*prev_button, *target_button, lookup) + "A";
            *prev_button = *target_button;
            Some(seq.chars().collect::<Vec<_>>())
        })
        .collect::<Vec<_>>()
}

fn solve(code: &Vec<char>, num_robots: usize) -> u64 {
    let (keypad_lookup, dirpad_lookup) = create_lookups();

    let optimal_sequence = get_optimal_sequence(code, &keypad_lookup);
    let mut freq = HashMap::<Vec<_>, u64>::new();
    for seq in optimal_sequence {
        *freq.entry(seq).or_default() += 1;
    }

    for _ in 0..num_robots {
        let mut next_freq = HashMap::<Vec<_>, u64>::new();
        for (curr_layer_seq, curr_layer_freq) in &freq {
            let next_layer_optimal_sequence = get_optimal_sequence(&curr_layer_seq, &dirpad_lookup);

            let mut local_freq = HashMap::<Vec<_>, u64>::new();
            for next_layer_seq in next_layer_optimal_sequence {
                *local_freq.entry(next_layer_seq).or_default() += 1;
            }

            for (next_layer_seq, next_layer_freq) in local_freq {
                *next_freq.entry(next_layer_seq).or_default() += curr_layer_freq * next_layer_freq;
            }
        }
        freq = next_freq;
    }

    freq.iter().map(|(seq, frq)| seq.len() as u64 * frq).sum()
}

fn solve_1(input: Vec<Vec<char>>) {
    let ans: u64 = input
        .iter()
        .map(|code| {
            solve(code, 2)
                * code
                    .iter()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
        })
        .sum();

    println!("{ans}");
}

fn solve_2(input: Vec<Vec<char>>) {
    let ans: u64 = input
        .iter()
        .map(|code| {
            solve(code, 25)
                * code
                    .iter()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<u64>()
                    .unwrap()
        })
        .sum();

    println!("{ans}");
}

fn main() {
    let input: Vec<_> = fs::read_to_string("input.txt")
        .unwrap()
        .split_terminator('\n')
        .map(|x| x.chars().collect())
        .collect();

    solve_1(input.clone());
    solve_2(input.clone());
}
