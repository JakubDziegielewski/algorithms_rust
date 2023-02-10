use std::vec;

fn main() {
    const MAX: u32 = 10;
    let sorted = counting_sort(&[10, 2, 1, 10, 9, 2, 4, 1, 1], MAX);
    assert_eq!(vec![1, 1, 1, 2, 2, 4, 9, 10, 10], sorted);
}

fn counting_sort(a: &[u32], max: u32) -> Vec<u32>{
    let mut counting_vec = vec![0; max as usize + 1];

    for val in a{
        let ind = *val as usize;  
        counting_vec[ind] = counting_vec[ind] + 1;

    }

    let mut sorted: Vec<u32> = Vec::new();

    for i in 0..=max{
        sorted.append(&mut vec![i; counting_vec[i as usize]]);
    }
    sorted
}