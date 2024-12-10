use std::{collections::VecDeque, fs};

const NEXTS4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn walk(input: Vec<Vec<u32>>, allow_revisits: bool) -> u32 {
    let (n, m) = (input.len(), input[0].len());

    let mut ans: u32 = 0;
    for i in 0..n {
        for j in 0..m {
            if input[i][j] == 0 {
                let mut vis = vec![vec![false; m]; n];
                vis[i][j] = true;
                let mut q = VecDeque::<(usize, usize)>::new();
                q.push_back((i, j));
                while !q.is_empty() {
                    let (r, c) = q.pop_front().unwrap();
                    if input[r][c] == 9 {
                        ans += 1;
                        continue;
                    }
                    for (rr, cc) in NEXTS4 {
                        let (nr, nc) = (r.wrapping_add_signed(rr), c.wrapping_add_signed(cc));
                        if nr < n
                            && nc < m
                            && input[nr][nc] == input[r][c] + 1
                            && (!vis[nr][nc] || allow_revisits)
                        {
                            vis[nr][nc] = true;
                            q.push_back((nr, nc));
                        }
                    }
                }
            }
        }
    }

    return ans;
}

fn solve_1(input: Vec<Vec<u32>>) {
    let ans = walk(input, false);

    println!("{ans}")
}

fn solve_2(input: Vec<Vec<u32>>) {
    let ans = walk(input, true);

    println!("{ans}")
}

fn main() {
    let input: Vec<Vec<u32>> = fs::read_to_string("input.txt")
        .unwrap()
        .split_terminator('\n')
        .map(|x| x.chars().map(|c| c.to_string().parse().unwrap()).collect())
        .collect();

    solve_1(input.clone());
    solve_2(input.clone());
}
