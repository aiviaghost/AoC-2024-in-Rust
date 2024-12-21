use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

const NEXTS4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn bfs(grid: &Vec<Vec<char>>, start: (usize, usize)) -> Vec<Vec<usize>> {
    let (n, m) = (grid.len(), grid[0].len());

    let mut dist = vec![vec![usize::MAX; m]; n];
    dist[start.0][start.1] = 0;
    let mut q = VecDeque::new();
    q.push_back(start);

    while let Some((r, c)) = q.pop_front() {
        for (rr, cc) in NEXTS4 {
            let (nr, nc) = (r.wrapping_add_signed(rr), c.wrapping_add_signed(cc));
            if nr < n && nc < m && grid[nr][nc] != '#' && dist[nr][nc] == usize::MAX {
                dist[nr][nc] = dist[r][c] + 1;
                q.push_back((nr, nc));
            }
        }
    }

    return dist;
}

fn solve(grid: Vec<Vec<char>>, cheat_duration: usize) -> u32 {
    let (n, m) = (grid.len(), grid[0].len());

    let mut start = (0, 0);
    let mut end = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'S' {
                start = (i, j);
            } else if grid[i][j] == 'E' {
                end = (i, j);
            }
        }
    }
    let start = start;
    let end = end;

    let dist_from_start = bfs(&grid, start);
    let dist_from_end = bfs(&grid, end);

    let mut ans: u32 = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] != '#' {
                let mut dist = HashMap::<(usize, usize), usize>::new();
                dist.entry((i, j)).or_default();
                let mut q = VecDeque::new();
                q.push_back((i, j));

                while let Some((r, c)) = q.pop_front() {
                    let curr_dist = *dist.get(&(r, c)).unwrap();

                    if grid[r][c] != '#' {
                        let shortcut_dist = dist_from_start[i][j] + curr_dist + dist_from_end[r][c];
                        if shortcut_dist < dist_from_start[end.0][end.1]
                            && dist_from_start[end.0][end.1] - shortcut_dist >= 100
                        {
                            ans += 1;
                        }
                    }

                    if curr_dist == cheat_duration {
                        continue;
                    }

                    for (rr, cc) in NEXTS4 {
                        let (nr, nc) = (r.wrapping_add_signed(rr), c.wrapping_add_signed(cc));
                        if nr < n && nc < m && !dist.contains_key(&(nr, nc)) {
                            let new_dist = curr_dist + 1;
                            if new_dist <= cheat_duration {
                                dist.insert((nr, nc), new_dist);
                                q.push_back((nr, nc));
                            }
                        }
                    }
                }
            }
        }
    }

    ans
}

fn solve_1(grid: Vec<Vec<char>>) {
    let ans = solve(grid, 2);

    println!("{ans}");
}

fn solve_2(grid: Vec<Vec<char>>) {
    let ans = solve(grid, 20);

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
