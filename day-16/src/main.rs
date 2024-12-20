use std::{
    cmp,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
};

const NEXTS4: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

fn solve_1(grid: Vec<Vec<char>>) {
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

    let mut cost = NEXTS4
        .iter()
        .map(|dir| (dir, vec![vec![u64::MAX; m]; n]))
        .collect::<HashMap<_, _>>();
    cost.get_mut(&NEXTS4[0]).unwrap()[start.0][start.1] = 0;

    let mut pq = BinaryHeap::<cmp::Reverse<_>>::new();
    pq.push(cmp::Reverse((0, NEXTS4[0], start)));

    while let Some(cmp::Reverse((curr_cost, curr_dir, (r, c)))) = pq.pop() {
        if cost[&curr_dir][r][c] < curr_cost {
            continue;
        }

        let dirs = [
            curr_dir,
            (curr_dir.1, curr_dir.0),
            (-curr_dir.1, -curr_dir.0),
            (-curr_dir.0, -curr_dir.1),
        ];
        let costs = [1, 1001, 1001, 2001];

        for (dir, edge_cost) in dirs.into_iter().zip(costs.into_iter()) {
            let (rr, cc) = (r.wrapping_add_signed(dir.0), c.wrapping_add_signed(dir.1));
            let alt_cost = curr_cost + edge_cost;
            if rr < n && cc < m && grid[rr][cc] != '#' && alt_cost < cost[&dir][rr][cc] {
                cost.get_mut(&dir).unwrap()[rr][cc] = alt_cost;
                pq.push(cmp::Reverse((alt_cost, dir, (rr, cc))));
            }
        }
    }

    let ans = NEXTS4
        .iter()
        .map(|dir| cost[dir][end.0][end.1])
        .min()
        .unwrap();

    println!("{ans}");
}

fn solve_2(grid: Vec<Vec<char>>) {
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

    let mut cost = NEXTS4
        .iter()
        .map(|dir| (dir, vec![vec![u64::MAX; m]; n]))
        .collect::<HashMap<_, _>>();
    cost.get_mut(&NEXTS4[0]).unwrap()[start.0][start.1] = 0;

    let mut pq = BinaryHeap::<cmp::Reverse<_>>::new();
    pq.push(cmp::Reverse((0, NEXTS4[0], start)));

    let mut par = HashMap::<
        ((isize, isize), (usize, usize)),
        Vec<(u64, (isize, isize), (usize, usize))>,
    >::new();

    while let Some(cmp::Reverse((curr_cost, curr_dir, (r, c)))) = pq.pop() {
        if cost[&curr_dir][r][c] < curr_cost {
            continue;
        }

        let dirs = [
            curr_dir,
            (curr_dir.1, curr_dir.0),
            (-curr_dir.1, -curr_dir.0),
            (-curr_dir.0, -curr_dir.1),
        ];
        let costs = [1, 1001, 1001, 2001];

        for (dir, edge_cost) in dirs.into_iter().zip(costs.into_iter()) {
            let (rr, cc) = (r.wrapping_add_signed(dir.0), c.wrapping_add_signed(dir.1));
            let alt_cost = curr_cost + edge_cost;
            if rr < n && cc < m && grid[rr][cc] != '#' {
                par.entry((dir, (rr, cc)))
                    .or_default()
                    .push((edge_cost, curr_dir, (r, c)));
                if alt_cost < cost[&dir][rr][cc] {
                    cost.get_mut(&dir).unwrap()[rr][cc] = alt_cost;
                    pq.push(cmp::Reverse((alt_cost, dir, (rr, cc))));
                }
            }
        }
    }

    let mut optimal_positions = HashSet::new();
    let mut q = VecDeque::new();

    let min_cost = NEXTS4
        .iter()
        .map(|dir| cost[dir][end.0][end.1])
        .min()
        .unwrap();
    for dir in NEXTS4 {
        if cost[&dir][end.0][end.1] == min_cost {
            q.push_back((dir, end));
        }
    }

    while let Some((dir, pos)) = q.pop_front() {
        optimal_positions.insert(pos);

        for &(edge_cost, par_dir, (pr, pc)) in par.get(&(dir, pos)).or(Some(&vec![])).unwrap() {
            if cost.get(&par_dir).unwrap()[pr][pc] + edge_cost
                == cost.get(&dir).unwrap()[pos.0][pos.1]
            {
                q.push_back((par_dir, (pr, pc)));
            }
        }
    }

    let ans = optimal_positions.len();

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
