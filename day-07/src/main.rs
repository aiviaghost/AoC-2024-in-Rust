use std::fs;

fn _solve(target: u64, idx: usize, nums: &Vec<u64>, acc: u64, part2: bool) -> Option<u64> {
    if idx == nums.len() {
        return if acc == target { Some(target) } else { None };
    }

    _solve(target, idx + 1, nums, acc + nums[idx], part2)
        .or(_solve(target, idx + 1, nums, acc * nums[idx], part2))
        .or_else(|| {
            if part2 {
                _solve(
                    target,
                    idx + 1,
                    nums,
                    acc * 10u64.pow((nums[idx] as f64).log10() as u32 + 1) + nums[idx],
                    part2,
                )
            } else {
                None
            }
        })
}

fn solve(target: u64, nums: &Vec<u64>, part2: bool) -> Option<u64> {
    _solve(target, 1, nums, nums[0], part2)
}

fn get_data(input: Vec<String>) -> Vec<(u64, Vec<u64>)> {
    input
        .iter()
        .map(|line| {
            let (target, nums) = line.split_once(": ").unwrap();
            let target = target.parse::<u64>().unwrap();
            let nums = nums
                .split(" ")
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            (target, nums)
        })
        .collect()
}

fn solve_1(input: Vec<String>) {
    let ans: u64 = get_data(input)
        .into_iter()
        .filter_map(|(target, nums)| solve(target, &nums, false))
        .sum();

    println!("{ans}");
}

fn solve_2(input: Vec<String>) {
    let ans: u64 = get_data(input)
        .into_iter()
        .filter_map(|(target, nums)| solve(target, &nums, true))
        .sum();

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
