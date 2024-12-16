use std::{fs, io::stdin};

use regex::Regex;

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
    let (n, m) = (103, 101);

    let re = Regex::new(r"(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut positions: Vec<(isize, isize)> = vec![];
    let mut velocities: Vec<(isize, isize)> = vec![];
    for line in input {
        let (_, [px, py, vx, vy]) = re.captures(&line).unwrap().extract();

        let px = px.parse::<isize>().unwrap();
        let py = py.parse::<isize>().unwrap();
        positions.push((px, py));

        let vx = vx.parse::<isize>().unwrap();
        let vy = vy.parse::<isize>().unwrap();
        velocities.push((vx, vy));
    }

    let mut quadrants = vec![0u32; 4];
    for ((px, py), (vx, vy)) in positions.into_iter().zip(velocities.iter()) {
        let px = (px + 100 * vx).rem_euclid(m);
        let py = (py + 100 * vy).rem_euclid(n);

        if px < m / 2 && py < n / 2 {
            quadrants[0] += 1;
        } else if px > m / 2 && py < n / 2 {
            quadrants[1] += 1;
        } else if px < m / 2 && py > n / 2 {
            quadrants[2] += 1;
        } else if px > m / 2 && py > n / 2 {
            quadrants[3] += 1;
        }
    }

    let ans: u32 = quadrants.iter().product();

    println!("{ans}")
}

fn solve_2(input: Vec<String>) {
    let (n, m) = (103, 101);

    let re = Regex::new(r"(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();

    let mut positions: Vec<(isize, isize)> = vec![];
    let mut velocities: Vec<(isize, isize)> = vec![];
    for line in input {
        let (_, [px, py, vx, vy]) = re.captures(&line).unwrap().extract();

        let px = px.parse::<isize>().unwrap();
        let py = py.parse::<isize>().unwrap();
        positions.push((px, py));

        let vx = vx.parse::<isize>().unwrap();
        let vy = vy.parse::<isize>().unwrap();
        velocities.push((vx, vy));
    }

    let input = stdin();
    let mut counter: u32 = 0;
    loop {
        counter += 1;
        let mut grid = vec![vec![0u32; m]; n];
        for ((px, py), (vx, vy)) in positions.iter_mut().zip(velocities.iter()) {
            *px = (*px + vx).rem_euclid(m as isize);
            *py = (*py + vy).rem_euclid(n as isize);

            grid[*py as usize][*px as usize] += 1;
        }

        let mut ok = false;
        for i in 0..n {
            for j in 0..m {
                if grid[i][j] > 0 {
                    'outer: for (dx, dy) in NEXTS8 {
                        let mut curr = (i, j);
                        for _ in 0..5 {
                            let next = (
                                (curr.0 as isize + dy).rem_euclid(m as isize) as usize,
                                (curr.1 as isize + dx).rem_euclid(m as isize) as usize,
                            );
                            if grid[next.0][next.1] == 0 {
                                continue 'outer;
                            }
                            curr = next;
                        }
                        ok = true;
                    }
                }
            }
        }

        if ok {
            println!("Counter: {}", counter);
            for line in grid {
                println!(
                    "{}",
                    line.iter()
                        .map(|&x| if x > 0 { "#" } else { " " })
                        .collect::<Vec<_>>()
                        .concat()
                )
            }
            let mut next = String::new();
            input.read_line(&mut next).expect("Failed reading input!");
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
