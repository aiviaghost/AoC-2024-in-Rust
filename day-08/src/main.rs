use std::{collections::HashMap, fs};

fn solve_1(input: Vec<Vec<char>>) {
    let mut ans: u32 = 0;

    let (n, m) = (input.len(), input[0].len());

    let mut tower_positions: Vec<(usize, usize)> = vec![];
    for i in 0..n {
        for j in 0..m {
            if input[i][j] != '.' {
                tower_positions.push((i, j));
            }
        }
    }

    for i in 0..n {
        for j in 0..m {
            for &(tr, tc) in &tower_positions {
                if tr == i && tc == j {
                    continue;
                }

                let diff_r = tr as isize - i as isize;
                let diff_c = tc as isize - j as isize;

                let nxt_r = tr.wrapping_add_signed(diff_r);
                let nxt_c = tc.wrapping_add_signed(diff_c);
                if nxt_r < n && nxt_c < m {
                    if input[nxt_r][nxt_c] == input[tr][tc] {
                        ans += 1;
                        break;
                    }
                }
            }
        }
    }

    println!("{ans}");
}

fn solve_2(input: Vec<Vec<char>>) {
    let mut ans: u32 = 0;

    let (n, m) = (input.len(), input[0].len());

    let mut tower_positions: Vec<(usize, usize)> = vec![];
    let mut counter = HashMap::<char, u32>::new();
    for i in 0..n {
        for j in 0..m {
            if input[i][j] != '.' {
                *counter.entry(input[i][j]).or_default() += 1;
                tower_positions.push((i, j));
            }
        }
    }

    let eps: f64 = 0.00001;
    for i in 0..n {
        for j in 0..m {
            if input[i][j] != '.' {
                continue;
            }
            'outer: for &(tr, tc) in &tower_positions {
                if tr == i && tc == j {
                    continue;
                }

                let mut diff_r = (tr as isize - i as isize) as f64;
                let mut diff_c = (tc as isize - j as isize) as f64;
                let h = diff_r.hypot(diff_c);
                diff_r /= h;
                diff_c /= h;

                for &(tr2, tc2) in &tower_positions {
                    if tr2 == i && tc2 == j {
                        continue;
                    }

                    if tr == tr2 && tc == tc2 {
                        continue;
                    }

                    if input[tr][tc] != input[tr2][tc2] {
                        continue;
                    }

                    let mut diff_r2 = (tr2 as isize - tr as isize) as f64;
                    let mut diff_c2 = (tc2 as isize - tc as isize) as f64;
                    let h = diff_r2.hypot(diff_c2);
                    diff_r2 /= h;
                    diff_c2 /= h;

                    if ((diff_r - diff_r2).abs() < eps && (diff_c - diff_c2).abs() < eps)
                        || ((diff_r + diff_r2).abs() < eps && (diff_c + diff_c2).abs() < eps)
                    {
                        ans += 1;
                        break 'outer;
                    }
                }
            }
        }
    }

    for v in counter.values() {
        if v > &1 {
            ans += v;
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
