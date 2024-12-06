use std::{collections::HashSet, fs};

const NEXTS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn solve_1(input: Vec<Vec<char>>) {
    let (n, m) = (input.len(), input[0].len());

    let (mut cx, mut cy) = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if input[i][j] == '^' {
                cx = j;
                cy = i;
            }
        }
    }

    let mut dir_idx = 0;

    let mut vis = HashSet::<(usize, usize)>::new();
    vis.insert((cx, cy));

    loop {
        let nx = cx.wrapping_add_signed(NEXTS[dir_idx].0);
        let ny = cy.wrapping_add_signed(NEXTS[dir_idx].1);
        if nx < m && ny < n {
            if input[ny][nx] == '#' {
                dir_idx = (dir_idx + 1) % NEXTS.len();
            } else {
                cx = nx;
                cy = ny;
                vis.insert((cx, cy));
            }
        } else {
            break;
        }
    }

    println!("{}", vis.len());
}

fn solve_2(mut input: Vec<Vec<char>>) {
    let (n, m) = (input.len(), input[0].len());

    let (mut sx, mut sy) = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if input[i][j] == '^' {
                sx = j;
                sy = i;
            }
        }
    }

    let mut ans: u32 = 0;
    for i in 0..n {
        for j in 0..m {
            if input[i][j] != '.' {
                continue;
            }
            input[i][j] = '#';

            let (mut cx, mut cy) = (sx, sy);
            let mut dir_idx = 0;

            let mut vis = HashSet::<(usize, usize, usize)>::new();
            vis.insert((cx, cy, dir_idx));

            loop {
                let nx = cx.wrapping_add_signed(NEXTS[dir_idx].0);
                let ny = cy.wrapping_add_signed(NEXTS[dir_idx].1);
                if nx < m && ny < n {
                    if input[ny][nx] == '#' {
                        dir_idx = (dir_idx + 1) % NEXTS.len();
                    } else {
                        cx = nx;
                        cy = ny;
                        if vis.contains(&(cx, cy, dir_idx)) {
                            ans += 1;
                            break;
                        }
                        vis.insert((cx, cy, dir_idx));
                    }
                } else {
                    break;
                }
            }

            input[i][j] = '.';
        }
    }

    println!("{ans}");
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
