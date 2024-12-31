use std::fs;

fn solve_1(input: Vec<Vec<Vec<u8>>>) {
    let (locks, keys): (Vec<_>, Vec<_>) = input
        .iter()
        .partition(|grid| grid[0].iter().all(|&c| c == b'#'));

    let mut ans: u32 = 0;

    for lock in &locks {
        for key in &keys {
            if !lock
                .into_iter()
                .flatten()
                .zip(key.into_iter().flatten())
                .map(|(a, b)| a + b)
                .any(|x| x == 2 * b'#')
            {
                ans += 1;
            }
        }
    }

    println!("{ans}");
}

fn solve_2(input: Vec<Vec<Vec<u8>>>) {
    println!("[Deliver The Chronicle]");
}

fn main() {
    let input: Vec<_> = fs::read_to_string("input.txt")
        .unwrap()
        .trim_end()
        .split_terminator("\n\n")
        .map(|x| x.split("\n").map(|s| s.bytes().collect()).collect())
        .collect();

    solve_1(input.clone());
    solve_2(input.clone());
}
