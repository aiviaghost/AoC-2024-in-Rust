use std::{collections::VecDeque, fs};

const NEXTS4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const NEXTS8: [(isize, isize); 8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
];

fn solve_1(input: Vec<String>) {
    let bytes = input
        .iter()
        .map(|line| {
            let mut data = line.split(",");
            (
                data.next().unwrap().parse().unwrap(),
                data.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();

    let (n, m) = (71, 71);
    let mut grid = vec![vec![false; m]; n];

    for &(r, c) in &bytes[..1024] {
        grid[r][c] = true;
    }

    let mut q = VecDeque::<(usize, usize)>::new();
    q.push_back((0, 0));
    let mut dist = vec![vec![usize::MAX; m]; n];
    dist[0][0] = 0;

    while let Some((r, c)) = q.pop_front() {
        for (rr, cc) in NEXTS4 {
            let (nr, nc) = (r.wrapping_add_signed(rr), c.wrapping_add_signed(cc));
            if nr < n && nc < m && !grid[nr][nc] && dist[nr][nc] == usize::MAX {
                dist[nr][nc] = dist[r][c] + 1;
                q.push_back((nr, nc));
            }
        }
    }

    println!("{}", dist[n - 1][m - 1]);
}

fn solve_2(input: Vec<String>) {
    let bytes = input
        .iter()
        .map(|line| {
            let mut data = line.split(",");
            (
                data.next().unwrap().parse().unwrap(),
                data.next().unwrap().parse().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>();

    let (n, m) = (71, 71);
    let mut grid = vec![vec![false; m]; n];

    for &(r, c) in &bytes {
        grid[r][c] = true;
        let mut q = VecDeque::<(usize, usize)>::new();
        q.push_back((0, 0));
        let mut dist = vec![vec![usize::MAX; m]; n];
        dist[0][0] = 0;

        while let Some((r, c)) = q.pop_front() {
            for (rr, cc) in NEXTS4 {
                let (nr, nc) = (r.wrapping_add_signed(rr), c.wrapping_add_signed(cc));
                if nr < n && nc < m && !grid[nr][nc] && dist[nr][nc] == usize::MAX {
                    dist[nr][nc] = dist[r][c] + 1;
                    q.push_back((nr, nc));
                }
            }
        }

        if dist[n - 1][m - 1] == usize::MAX {
            println!("{},{}", r, c);
            break;
        }
    }
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
