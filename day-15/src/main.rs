use std::{
    collections::{HashMap, VecDeque},
    fs,
};

const NEXTS4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn solve_1(input: Vec<String>) {
    let move_to_dir = "><v^^".chars().zip(NEXTS4).collect::<HashMap<_, _>>();

    let mut data = input.split(|line| line.is_empty());
    let mut grid = data
        .next()
        .unwrap()
        .iter()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let moves = data.next().unwrap().join("").chars().collect::<Vec<_>>();

    let (n, m) = (grid.len(), grid[0].len());

    let mut robot = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '@' {
                robot = (i, j);
            }
        }
    }

    for movement in &moves {
        let dir = move_to_dir[movement];
        let (nr, nc) = (
            robot.0.wrapping_add_signed(dir.0),
            robot.1.wrapping_add_signed(dir.1),
        );

        let mut can_move = false;
        let mut curr = (nr, nc);
        loop {
            if grid[curr.0][curr.1] == '.' {
                can_move = true;
                break;
            } else if grid[curr.0][curr.1] == '#' {
                break;
            }

            curr = (
                curr.0.wrapping_add_signed(dir.0),
                curr.1.wrapping_add_signed(dir.1),
            );
        }

        if can_move {
            let mut q = VecDeque::new();
            q.push_back((robot.0, robot.1, '@'));
            grid[robot.0][robot.1] = '.';

            while let Some((r, c, t)) = q.pop_front() {
                let (nr, nc) = (r.wrapping_add_signed(dir.0), c.wrapping_add_signed(dir.1));

                if grid[nr][nc] == 'O' {
                    q.push_back((nr, nc, 'O'));
                }

                grid[nr][nc] = t;
            }

            robot = (nr, nc);
        }
    }

    let mut ans: usize = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 'O' {
                ans += 100 * i + j;
            }
        }
    }

    println!("{ans}")
}

fn try_move_vertical(
    grid: &mut Vec<Vec<char>>,
    pos: (usize, usize),
    dir: (isize, isize),
    is_other_side: bool,
    updates: &mut HashMap<(usize, usize), char>,
) -> bool {
    let (nr, nc) = (
        pos.0.wrapping_add_signed(dir.0),
        pos.1.wrapping_add_signed(dir.1),
    );

    let this_type = *updates.get(&pos).or(Some(&grid[pos.0][pos.1])).unwrap();

    if this_type == '#' {
        return false;
    }

    if this_type == '.' {
        return true;
    }

    let can_move_next = try_move_vertical(grid, (nr, nc), dir, false, updates);

    if can_move_next {
        if this_type == '[' || this_type == ']' {
            let other_side = if this_type == '[' { nc + 1 } else { nc - 1 };

            if !is_other_side && !try_move_vertical(grid, (pos.0, other_side), dir, true, updates) {
                return false;
            }
        }

        updates.insert((nr, nc), this_type);
        updates.insert(pos, '.');

        return true;
    }

    false
}

fn solve_2(input: Vec<String>) {
    let move_to_dir = "><v^^".chars().zip(NEXTS4).collect::<HashMap<_, _>>();

    let mut data = input.split(|line| line.is_empty());
    let mut grid = data
        .next()
        .unwrap()
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => "@.".to_string(),
                    'O' => "[]".to_string(),
                    c => format!("{}{}", c, c),
                })
                .collect::<Vec<_>>()
                .join("")
                .chars()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let moves = data.next().unwrap().join("").chars().collect::<Vec<_>>();

    let (n, m) = (grid.len(), grid[0].len());

    let mut robot = (0, 0);
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '@' {
                robot = (i, j);
            }
        }
    }

    for movement in &moves {
        let dir = move_to_dir[movement];

        let next_pos = (
            robot.0.wrapping_add_signed(dir.0),
            robot.1.wrapping_add_signed(dir.1),
        );

        if movement == &'<' || movement == &'>' {
            let mut can_move = false;
            let mut curr = next_pos;
            loop {
                if grid[curr.0][curr.1] == '.' {
                    can_move = true;
                    break;
                } else if grid[curr.0][curr.1] == '#' {
                    break;
                }

                curr = (
                    curr.0.wrapping_add_signed(dir.0),
                    curr.1.wrapping_add_signed(dir.1),
                );
            }

            if can_move {
                let mut q = VecDeque::new();
                q.push_back((robot.0, robot.1, '@'));
                grid[robot.0][robot.1] = '.';

                while let Some((r, c, t)) = q.pop_front() {
                    let (nr, nc) = (r.wrapping_add_signed(dir.0), c.wrapping_add_signed(dir.1));

                    if grid[nr][nc] == '[' {
                        q.push_back((nr, nc, '['));
                    } else if grid[nr][nc] == ']' {
                        q.push_back((nr, nc, ']'));
                    }

                    grid[nr][nc] = t;
                }

                robot = next_pos;
            }
        } else {
            let mut updates = HashMap::new();
            if try_move_vertical(&mut grid, robot, dir, false, &mut updates) {
                for ((r, c), t) in updates {
                    grid[r][c] = t;
                }
                robot = next_pos;
            }
        }
    }

    let mut ans: usize = 0;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == '[' {
                ans += 100 * i + j;
            }
        }
    }

    println!("{ans}");
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
