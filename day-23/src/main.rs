use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

fn solve_1(input: Vec<String>) {
    let mut adj = HashMap::<_, Vec<_>>::new();
    for line in &input {
        let (a, b) = line.split_once("-").unwrap();
        adj.entry(a).or_default().push(b);
        adj.entry(b).or_default().push(a);
    }

    let mut ans = 0;
    for start in adj.keys() {
        let mut dist = HashMap::new();
        dist.insert(start, 0);
        let mut q = VecDeque::new();
        q.push_back((start, start.starts_with("t")));

        while let Some((curr, curr_is_valid)) = q.pop_front() {
            let curr_dist = *dist.get(curr).unwrap();
            for next in adj.get(curr).unwrap() {
                if !dist.contains_key(next) {
                    if curr_dist + 1 <= 2 {
                        dist.insert(next, curr_dist + 1);
                        q.push_back((next, curr_is_valid || next.starts_with("t")));
                    }
                } else if (curr_is_valid || next.starts_with("t"))
                    && curr_dist + 1 + dist.get(next).unwrap() == 3
                {
                    ans += 1;
                }
            }
        }
    }

    ans /= 6;

    println!("{ans}");
}

fn solve_2(input: Vec<String>) {
    let mut adj = HashMap::<_, HashSet<_>>::new();
    for line in &input {
        let (a, b) = line.split_once("-").unwrap();
        adj.entry(a).or_default().insert(b);
        adj.entry(b).or_default().insert(a);
    }

    // let max_deg = adj.values().map(|x| x.len()).max().unwrap();
    // assert!(max_deg == 13);

    let mut ans = HashSet::new();
    for (&start, neighbours) in &adj {
        for mask in 0..(1u32 << neighbours.len()) {
            if mask.count_ones() + 1 <= ans.len() as u32 {
                continue;
            }

            let mut connected = HashSet::from([start]);
            for (i, &next) in neighbours.iter().enumerate() {
                if ((mask >> i) & 1) == 1 {
                    connected.insert(next);
                }
            }

            let is_clique = connected
                .iter()
                .map(|node| {
                    adj.get(node).unwrap().intersection(&connected).count() == connected.len() - 1
                })
                .all(|p| p);

            if is_clique {
                ans = connected;
            }
        }
    }

    let mut ans = ans.into_iter().collect::<Vec<_>>();
    ans.sort_unstable();
    let ans = ans.join(",");

    println!("{ans}");
}

fn main() {
    let input: Vec<_> = fs::read_to_string("input.txt")
        .unwrap()
        .split_terminator('\n')
        .map(|x| x.to_string())
        .collect();

    solve_1(input.clone());
    solve_2(input.clone());
}
