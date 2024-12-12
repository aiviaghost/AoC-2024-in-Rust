use std::{collections::VecDeque, fs};

const NEXTS4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn solve_1(input: Vec<Vec<char>>) {
    let (n, m) = (input.len(), input[0].len());

    let mut vis = vec![vec![false; m]; n];

    let mut ans: u32 = 0;
    for i in 0..n {
        for j in 0..m {
            if !vis[i][j] {
                vis[i][j] = true;

                let mut area: u32 = 0;
                let mut perimiter: u32 = 0;

                let mut q = VecDeque::new();
                q.push_back((i, j));

                while let Some((r, c)) = q.pop_front() {
                    area += 1;
                    for (rr, cc) in NEXTS4 {
                        let (nr, nc) = (r.wrapping_add_signed(rr), c.wrapping_add_signed(cc));
                        if nr < n && nc < m && input[nr][nc] == input[i][j] {
                            if !vis[nr][nc] {
                                vis[nr][nc] = true;
                                q.push_back((nr, nc));
                            }
                        } else {
                            perimiter += 1;
                        }
                    }
                }

                ans += area * perimiter;
            }
        }
    }

    println!("{ans}")
}

fn solve_2(input: Vec<Vec<char>>) {
    let (n, m) = (input.len(), input[0].len());

    let mut vis = vec![vec![false; m]; n];

    let mut ans: u32 = 0;
    for i in 0..n {
        for j in 0..m {
            if !vis[i][j] {
                vis[i][j] = true;

                let mut area: u32 = 0;
                let mut perimiter = vec![vec![0u8; m + 2]; n + 2];

                let mut q = VecDeque::new();
                q.push_back((i, j));

                while let Some((r, c)) = q.pop_front() {
                    area += 1;
                    for (dir_idx, &(rr, cc)) in NEXTS4.iter().enumerate() {
                        let (nr, nc) = (r.wrapping_add_signed(rr), c.wrapping_add_signed(cc));
                        if nr < n && nc < m && input[nr][nc] == input[i][j] {
                            if !vis[nr][nc] {
                                vis[nr][nc] = true;
                                q.push_back((nr, nc));
                            }
                        } else {
                            perimiter[(r + 1).wrapping_add_signed(rr)]
                                [(c + 1).wrapping_add_signed(cc)] |= 1 << dir_idx;
                        }
                    }
                }

                let mut dir_vis = vec![vec![0u8; m + 2]; n + 2];
                let mut num_sides: u32 = 0;
                for dir_idx in 0..4 {
                    for ii in 0..n + 2 {
                        for jj in 0..m + 2 {
                            if (perimiter[ii][jj] & (1 << dir_idx)) > 0
                                && (dir_vis[ii][jj] & (1 << dir_idx)) == 0
                            {
                                num_sides += 1;

                                dir_vis[ii][jj] |= 1 << dir_idx;

                                let mut dir = (0, 0);
                                for (rr, cc) in NEXTS4 {
                                    let (nr, nc) =
                                        (ii.wrapping_add_signed(rr), jj.wrapping_add_signed(cc));

                                    if nr < n + 2
                                        && nc < m + 2
                                        && (perimiter[ii][jj] & perimiter[nr][nc] & (1 << dir_idx))
                                            > 0
                                    {
                                        dir = (rr, cc);
                                        break;
                                    }
                                }

                                if dir == (0, 0) {
                                    continue;
                                }

                                for dir in [dir, (dir.0 * -1, dir.1 * -1)] {
                                    let mut curr = (ii, jj);
                                    loop {
                                        curr = (
                                            curr.0.wrapping_add_signed(dir.0),
                                            curr.1.wrapping_add_signed(dir.1),
                                        );

                                        if curr.0 < n + 2
                                            && curr.1 < m + 2
                                            && (perimiter[ii][jj]
                                                & perimiter[curr.0][curr.1]
                                                & (1 << dir_idx))
                                                > 0
                                        {
                                            dir_vis[curr.0][curr.1] |= 1 << dir_idx;
                                        } else {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                ans += area * num_sides;
            }
        }
    }

    println!("{ans}")
}

fn main() {
    let input: Vec<Vec<char>> = fs::read_to_string("input.txt")
        .unwrap()
        .split_terminator('\n')
        .map(|x| x.chars().collect())
        .collect();

    solve_1(input.clone());
    solve_2(input.clone());
}
