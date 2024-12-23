use std::{
    collections::{HashMap, HashSet},
    fs,
};

const MOD: u64 = 16777216;

fn transform(mut x: u64) -> u64 {
    x = ((x << 6) ^ x) % MOD;
    x = ((x >> 5) ^ x) % MOD;
    x = ((x << 11) ^ x) % MOD;
    return x;
}

fn solve_1(input: Vec<u64>) {
    let ans: u64 = input
        .into_iter()
        .map(|x| (0..2000).fold(x, |acc, _| transform(acc)))
        .sum();

    println!("{ans}");
}

fn solve_2(input: Vec<u64>) {
    let mut lookups = vec![HashMap::new(); input.len()];
    let mut seen_seqs = HashSet::new();

    for (seller, lookup) in input.into_iter().zip(lookups.iter_mut()) {
        (0..2000)
            .scan(seller, |state, _| {
                let curr_state = *state;
                let ones_digit_curr = (curr_state % 10) as i64;
                let next_state = transform(curr_state);
                let ones_digit_next = (next_state % 10) as i64;
                *state = next_state;
                Some((ones_digit_next - ones_digit_curr, ones_digit_next))
            })
            .collect::<Vec<_>>()
            .windows(4)
            .map(|w| {
                let w: Vec<_> = w.into();
                let (seq, vals): (Vec<_>, Vec<_>) = w.into_iter().unzip();
                let seq = (seq[0], seq[1], seq[2], seq[3]);
                let price = vals[3];
                (seq, price)
            })
            .for_each(|(seq, price)| {
                if !lookup.contains_key(&seq) {
                    lookup.insert(seq, price);
                    seen_seqs.insert(seq);
                }
            });
    }

    let ans = seen_seqs
        .iter()
        .map(|seq| {
            lookups
                .iter()
                .filter_map(|lookup| lookup.get(seq))
                .sum::<i64>()
        })
        .max()
        .unwrap();

    println!("{ans}");
}

fn main() {
    let input: Vec<_> = fs::read_to_string("input.txt")
        .unwrap()
        .split_terminator('\n')
        .map(|x| x.parse().unwrap())
        .collect();

    solve_1(input.clone());
    solve_2(input.clone());
}
