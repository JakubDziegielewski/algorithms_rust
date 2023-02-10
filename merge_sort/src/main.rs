
fn main() {
let arr: &[i32] = &[18000, 10, -10, 21, 44, 82, 12, 2, -1];

let sorted: Vec<i32> = merge_sort(arr);

assert_eq!(Vec::from([-10, -1, 2, 10, 12, 21, 44, 82, 18000]), sorted);

println!("{:?}", sorted);

}

fn merge_sort(a: &[i32]) -> Vec<i32> {
    let n = a.len();

    if n == 1 {
        return Vec::from(a);
    }
    
    let mut sorted: Vec<i32> = Vec::new();
    let half_one: Vec<i32> = Vec::from(&a[..n / 2]);
    let half_two: Vec<i32> = Vec::from(&a[n / 2..]);

    let sorted_half_one = merge_sort(&half_one);
    let sorted_half_two = merge_sort(&half_two);

    let mut half_one_iter: usize = 0;
    let mut half_two_iter: usize = 0;

    while sorted.len() < n {
        if half_one_iter >= sorted_half_one.len() {
            sorted.push(sorted_half_two[half_two_iter]);
            half_two_iter = half_two_iter + 1;
        } else if half_two_iter >= sorted_half_two.len() {
            sorted.push(sorted_half_one[half_one_iter]);
            half_one_iter = half_one_iter + 1;
        } else if sorted_half_one[half_one_iter] > sorted_half_two[half_two_iter] {
            sorted.push(sorted_half_two[half_two_iter]);
            half_two_iter = half_two_iter + 1;
        } else {
            sorted.push(sorted_half_one[half_one_iter]);
            half_one_iter = half_one_iter + 1;
        }
    }
    sorted
}
