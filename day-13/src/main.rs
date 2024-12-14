use std::{fs, ops};

use regex::Regex;

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        (a, b) = (b, a.rem_euclid(b));
    }
    return a;
}

#[derive(Debug, Clone, Copy)]
struct Frac {
    nom: i64,
    denom: i64,
}

impl Frac {
    fn new(nom: i64, denom: i64) -> Self {
        let div = gcd(nom, denom);
        let mut nom = nom / div;
        let mut denom = denom / div;

        if denom < 0 {
            nom *= -1;
            denom *= -1;
        }

        Frac { nom, denom }
    }

    fn is_nonnegative(&self) -> bool {
        self.nom >= 0
    }

    fn is_integer(&self) -> bool {
        self.denom == 1
    }
}

impl ops::Add<Frac> for Frac {
    type Output = Frac;

    fn add(self, other: Frac) -> Frac {
        let nom = self.nom * other.denom + other.nom * self.denom;
        let denom = self.denom * other.denom;

        Frac::new(nom, denom)
    }
}

impl ops::Sub<Frac> for Frac {
    type Output = Frac;

    fn sub(self, other: Frac) -> Frac {
        let nom = self.nom * other.denom - other.nom * self.denom;
        let denom = self.denom * other.denom;

        Frac::new(nom, denom)
    }
}

impl ops::Mul<Frac> for Frac {
    type Output = Frac;

    fn mul(self, other: Frac) -> Frac {
        let nom = self.nom * other.nom;
        let denom = self.denom * other.denom;

        Frac::new(nom, denom)
    }
}

impl ops::Div<Frac> for Frac {
    type Output = Frac;

    fn div(self, other: Frac) -> Frac {
        let nom = self.nom * other.denom;
        let denom = self.denom * other.nom;

        Frac::new(nom, denom)
    }
}

/*
 * Solve Ax = b, where
 * A = [a11 a12]
 *     [a21 a22]
 * b = [b1 b2].T
 */
fn solve_equations(a11: i64, a12: i64, a21: i64, a22: i64, b1: i64, b2: i64) -> (Frac, Frac) {
    let a11 = Frac::new(a11, 1);
    let mut a12 = Frac::new(a12, 1);
    let a21 = Frac::new(a21, 1);
    let mut a22 = Frac::new(a22, 1);
    let mut b1 = Frac::new(b1, 1);
    let mut b2 = Frac::new(b2, 1);

    a12 = a12 / a11;
    b1 = b1 / a11;

    a22 = a22 - a21 * a12;
    b2 = b2 - a21 * b1;

    b2 = b2 / a22;

    b1 = b1 - a12 * b2;

    (b1, b2)
}

fn find_solution_cost(
    dx1: i64,
    dx2: i64,
    dy1: i64,
    dy2: i64,
    target_x: i64,
    target_y: i64,
) -> Option<u64> {
    let sol = solve_equations(dx1, dx2, dy1, dy2, target_x, target_y);

    let is_integer_solution = sol.0.is_integer() && sol.1.is_integer();
    let is_valid_solution = sol.0.is_nonnegative()
        && sol.1.is_nonnegative()
        && dx1 * sol.0.nom + dx2 * sol.1.nom == target_x
        && dy1 * sol.0.nom + dy2 * sol.1.nom == target_y;

    if is_integer_solution && is_valid_solution {
        Some((3 * sol.0.nom + sol.1.nom) as u64)
    } else {
        None
    }
}

fn solve_1(input: Vec<(i64, i64, i64, i64, i64, i64)>) {
    let ans: u64 = input
        .into_iter()
        .filter_map(|(dx1, dx2, dy1, dy2, target_x, target_y)| {
            find_solution_cost(dx1, dx2, dy1, dy2, target_x, target_y)
        })
        .sum();

    println!("{ans}")
}

fn solve_2(input: Vec<(i64, i64, i64, i64, i64, i64)>) {
    let ans: u64 = input
        .into_iter()
        .filter_map(|(dx1, dx2, dy1, dy2, target_x, target_y)| {
            find_solution_cost(
                dx1,
                dx2,
                dy1,
                dy2,
                target_x + 10000000000000,
                target_y + 10000000000000,
            )
        })
        .sum();

    println!("{ans}")
}

fn main() {
    let re_button = Regex::new(r"X(\+|=)(\d+), Y(\+|=)(\d+)").unwrap();
    let input: Vec<_> = fs::read_to_string("input.txt")
        .unwrap()
        .split_terminator('\n')
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .split(|line| line.is_empty())
        .map(|lines| {
            let button_a = &lines[0];
            let (_, [_, dx, _, dy]) = re_button.captures(button_a).unwrap().extract();
            let dx1 = dx.parse::<i64>().unwrap();
            let dy1 = dy.parse::<i64>().unwrap();

            let button_b = &lines[1];
            let (_, [_, dx, _, dy]) = re_button.captures(button_b).unwrap().extract();
            let dx2 = dx.parse::<i64>().unwrap();
            let dy2 = dy.parse::<i64>().unwrap();

            let prize = &lines[2];
            let (_, [_, target_x, _, target_y]) = re_button.captures(prize).unwrap().extract();
            let target_x = target_x.parse::<i64>().unwrap();
            let target_y = target_y.parse::<i64>().unwrap();

            (dx1, dx2, dy1, dy2, target_x, target_y)
        })
        .collect();

    solve_1(input.clone());
    solve_2(input.clone());
}
