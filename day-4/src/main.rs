use std::fs;

fn solve_1(input: Vec<String>) {
    let grid = input
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let xmas = "XMAS".chars().collect::<Vec<_>>();

    let (n, m) = (input.len(), input[0].len());

    let dirs: [(i32, i32); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    let mut ans = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == xmas[0] {
                for (dx, dy) in dirs {
                    let mut matches = 1;
                    let (mut cx, mut cy) = (j as i32, i as i32);
                    for idx in 1..=3 {
                        cx += dx;
                        cy += dy;
                        if 0 <= cx
                            && cx < m as i32
                            && 0 <= cy
                            && cy < n as i32
                            && grid[cy as usize][cx as usize] == xmas[idx]
                        {
                            matches += 1
                        }
                    }
                    if matches == 4 {
                        ans += 1;
                    }
                }
            }
        }
    }

    println!("{ans}")
}

fn solve_2(input: Vec<String>) {
    let grid = input
        .iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let (n, m) = (input.len(), input[0].len());

    let mut ans = 0;
    for i in 1..n - 1 {
        for j in 1..m - 1 {
            if grid[i][j] == 'A' {
                let cx1 = ((j as i32) - 1) as usize;
                let cy1 = ((i as i32) - 1) as usize;
                let cx2 = ((j as i32) + 1) as usize;
                let cy2 = ((i as i32) + 1) as usize;

                let cx3 = ((j as i32) + 1) as usize;
                let cy3 = ((i as i32) - 1) as usize;
                let cx4 = ((j as i32) - 1) as usize;
                let cy4 = ((i as i32) + 1) as usize;

                if (grid[cy1][cx1] == 'M'
                    && grid[cy2][cx2] == 'S'
                    && grid[cy3][cx3] == 'M'
                    && grid[cy4][cx4] == 'S')
                    || (grid[cy1][cx1] == 'S'
                        && grid[cy2][cx2] == 'M'
                        && grid[cy3][cx3] == 'M'
                        && grid[cy4][cx4] == 'S')
                    || (grid[cy1][cx1] == 'M'
                        && grid[cy2][cx2] == 'S'
                        && grid[cy3][cx3] == 'S'
                        && grid[cy4][cx4] == 'M')
                    || (grid[cy1][cx1] == 'S'
                        && grid[cy2][cx2] == 'M'
                        && grid[cy3][cx3] == 'S'
                        && grid[cy4][cx4] == 'M')
                {
                    ans += 1;
                }
            }
        }
    }

    println!("{ans}")
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
