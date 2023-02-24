use std::cmp::min;

fn main() {
    let sum: i32 = editing_distance("distance", "editing");

    println!("{sum}");
}

fn editing_distance(first_word: &str, second_word: &str) -> i32 {
    let mut matrix: Vec<Vec<i32>> = vec![vec![0; first_word.len() + 1]; second_word.len() + 1];
    let first_chars: Vec<char> = first_word.chars().collect();
    let second_chars: Vec<char> = second_word.chars().collect();
    for j in 1..=second_chars.len(){
        matrix[j][0] = j as i32;
    }
    for i in 1..=first_chars.len(){
        matrix[0][i] = i as i32;
    }

    for j in 1..=second_chars.len() {
        for i in 1..=first_chars.len() {
            let insertion: i32 = matrix[j][i - 1] + 1;
            let deletion: i32 = matrix[j - 1][i] + 1;
            let equal: i32 = matrix[j - 1][i - 1];
            let mismatch: i32 = matrix[j - 1][i - 1] + 1;
            
            if first_chars[i - 1] == second_chars[j - 1]{
                let mut min_val: i32 = min(insertion, deletion);
                min_val = min(min_val, equal);
                matrix[j][i] = min_val;
            }
            else {
                let mut min_val: i32 = min(insertion, deletion);
                min_val = min(min_val, mismatch);
                matrix[j][i] = min_val;
            }
        }
    }

    println!("{:?}", matrix);
    matrix[second_word.len()][first_word.len()]
}
