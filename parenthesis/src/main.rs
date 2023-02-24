use std::{
    cmp::{max, min},
    process::exit,
};

fn main() {
    let numbers: &[i32] = &[5, 8, 7, 4, 8, 9];
    let operators: &[char] = &['-', '+', '*', '-', '+'];

    let res: i32 = parenthesis(numbers, operators);
    println!("{res}");
}

fn min_max(
    i: usize,
    j: usize,
    operators: &[char],
    big_m: &Vec<Vec<i32>>,
    small_m: &Vec<Vec<i32>>,
) -> (i32, i32) {
    let mut min_val = i32::MAX;
    let mut max_val: i32 = i32::MIN;
    let mut a: i32;
    let mut b: i32;
    let mut c: i32;
    let mut d: i32;

    for k in i..j {
        if operators[k] == '+' {
            a = big_m[i][k] + big_m[k + 1][j];
            b = big_m[i][k] + small_m[k + 1][j];
            c = small_m[i][k] + big_m[k + 1][j];
            d = small_m[i][k] + small_m[k + 1][j];
        } else if operators[k] == '*' {
            a = big_m[i][k] * big_m[k + 1][j];
            b = big_m[i][k] * small_m[k + 1][j];
            c = small_m[i][k] * big_m[k + 1][j];
            d = small_m[i][k] * small_m[k + 1][j];
        } else if operators[k] == '-' {
            a = big_m[i][k] - big_m[k + 1][j];
            b = big_m[i][k] - small_m[k + 1][j];
            c = small_m[i][k] - big_m[k + 1][j];
            d = small_m[i][k] - small_m[k + 1][j];
        } else {
            println!("error");
            exit(1);
        }

        min_val = min(min_val, min(min(a, b), min(c, d)));
        max_val = max(max_val, max(max(a, b), max(c, d)));
    }

    (max_val, min_val)
}

fn parenthesis(numbers: &[i32], operators: &[char]) -> i32 {
    let mut big_m: Vec<Vec<i32>> = vec![vec![0; numbers.len()]; numbers.len()];
    let mut small_m: Vec<Vec<i32>> = big_m.clone();
    for i in 0..numbers.len() {
        big_m[i][i] = numbers[i];
        small_m[i][i] = numbers[i];
    }

    for s in 1..numbers.len() {
        for i in 0..numbers.len() - s {
            let j = i + s;
            (big_m[i][j], small_m[i][j]) = min_max(i, j, operators, &big_m, &small_m);
        }
    }
    big_m[0][numbers.len() - 1]
}
