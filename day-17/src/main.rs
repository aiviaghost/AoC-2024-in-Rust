use std::fs;

fn combo(reg_a: usize, reg_b: usize, reg_c: usize, operand: usize) -> usize {
    match operand {
        0..=3 => operand,
        4 => reg_a,
        5 => reg_b,
        6 => reg_c,
        _ => unreachable!(),
    }
}

fn solve_1(input: Vec<String>) {
    let mut input = input.split(|line| line.is_empty());
    let mut regs = input
        .next()
        .unwrap()
        .iter()
        .map(|line| line.split(" ").last().unwrap().parse().unwrap());
    let mut reg_a: usize = regs.next().unwrap();
    let mut reg_b: usize = regs.next().unwrap();
    let mut reg_c: usize = regs.next().unwrap();

    let instructions = input
        .next()
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    let mut out: Vec<usize> = vec![];

    let mut pc = 0;

    while pc < instructions.len() {
        match instructions[pc] {
            0 => {
                if pc + 1 == instructions.len() {
                    break;
                }
                reg_a /= 2usize.pow(combo(reg_a, reg_b, reg_c, instructions[pc + 1]) as u32);
                pc += 2;
            }
            1 => {
                if pc + 1 == instructions.len() {
                    break;
                }
                reg_b ^= instructions[pc + 1];
                pc += 2;
            }
            2 => {
                if pc + 1 == instructions.len() {
                    break;
                }
                reg_b = combo(reg_a, reg_b, reg_c, instructions[pc + 1]) % 8;
                pc += 2;
            }
            3 => {
                if reg_a == 0 {
                    pc += 2;
                    continue;
                }
                if pc + 1 == instructions.len() {
                    break;
                }
                pc = instructions[pc + 1];
            }
            4 => {
                if pc + 1 == instructions.len() {
                    break;
                }
                reg_b = reg_b ^ reg_c;
                pc += 2;
            }
            5 => {
                if pc + 1 == instructions.len() {
                    break;
                }
                out.push(combo(reg_a, reg_b, reg_c, instructions[pc + 1]) % 8);
                pc += 2;
            }
            6 => {
                if pc + 1 == instructions.len() {
                    break;
                }
                reg_b = reg_a / 2usize.pow(combo(reg_a, reg_b, reg_c, instructions[pc + 1]) as u32);
                pc += 2;
            }
            7 => {
                if pc + 1 == instructions.len() {
                    break;
                }
                reg_c = reg_a / 2usize.pow(combo(reg_a, reg_b, reg_c, instructions[pc + 1]) as u32);
                pc += 2;
            }
            _ => unreachable!(),
        }
    }

    println!(
        "{}",
        out.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn solve_rec(instructions: &Vec<usize>, idx: usize, reg_a: usize) -> Option<usize> {
    for a in 0..8 {
        let A = (reg_a << 3) | a;
        let mut B = A % 8;
        B = B ^ 5;
        let C = A >> B;
        B = B ^ 6;
        B = B ^ C;

        if B % 8 == instructions[idx] {
            if idx == 0 {
                return Some(A);
            } else if let Some(A) = solve_rec(instructions, idx - 1, A) {
                return Some(A);
            }
        }
    }

    None
}

/*
Decompiled program:

while A != 0:
    B = A % 8
    B = B ^ 5
    C = A >> B
    B = B ^ 6
    A = A >> 3
    B = B ^ C
    print(B % 8)
*/

fn solve_2(input: Vec<String>) {
    let mut input = input.split(|line| line.is_empty());
    let mut regs = input
        .next()
        .unwrap()
        .iter()
        .map(|line| line.split(" ").last().unwrap().parse().unwrap());
    let mut reg_a: usize = regs.next().unwrap();
    let mut reg_b: usize = regs.next().unwrap();
    let mut reg_c: usize = regs.next().unwrap();

    let instructions = input
        .next()
        .unwrap()
        .iter()
        .next()
        .unwrap()
        .split(" ")
        .last()
        .unwrap()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<usize>>();

    let ans = solve_rec(&instructions, instructions.len() - 1, 0).unwrap();

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
