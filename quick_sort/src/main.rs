fn main() {
    let a: &mut [i32] = &mut [2, 1, 0, 2, 4, 3];

    let len = a.len();
    let b: &mut Vec<i32> = &mut vec![0; len];
    b.clone_from_slice(&a[..]);
    quick_sort(&mut a[..], 0, len as i32 - 1);
    quick_sort_with_shorter_slicec(&mut b[..]);
    assert_eq!(a, b);
    println!("{:?}", a);
}

fn quick_sort(mut a: &mut [i32], l: i32, r: i32) {
    //clasic method
    if l >= r {
        return;
    }

    let mut j = l;

    for i in l + 1..r + 1 {
        if a[i as usize] <= a[l as usize] {
            j = j + 1;
            (a[i as usize], a[j as usize]) = (a[j as usize], a[i as usize]);
        }
    }

    (a[l as usize], a[j as usize]) = (a[j as usize], a[l as usize]);

    quick_sort(&mut a, l, j - 1);
    quick_sort(&mut a, j + 1, r);
}
fn quick_sort_with_shorter_slicec(a: &mut [i32]) {
    if a.len() <= 0 {
        return;
    }
    //recursive methods are only given the slice, that they are supposed to sort
    let len = a.len();
    let mut j = 0;
    for i in 1..len {
        if a[i as usize] <= a[0] {
            j = j + 1;
            (a[i as usize], a[j as usize]) = (a[j as usize], a[i as usize]);
        }
    }
    (a[0], a[j as usize]) = (a[j as usize], a[0]);

    quick_sort_with_shorter_slicec(&mut a[0..j as usize]);
    quick_sort_with_shorter_slicec(&mut a[j as usize + 1..len]);
}
fn _quick_sort_with_new_vector(a: &[i32], l: i32, r: i32) -> Vec<i32> {
    let mut sorted = Vec::from(a);

    if l >= r {
        return sorted;
    }

    let mut index = l;

    for i in l + 1..r + 1 {
        if sorted[i as usize] <= sorted[l as usize] {
            index = index + 1;
            (sorted[i as usize], sorted[index as usize]) =
                (sorted[index as usize], sorted[i as usize]);
        }
    }

    (sorted[l as usize], sorted[index as usize]) = (sorted[index as usize], sorted[l as usize]);

    sorted = _quick_sort_with_new_vector(&sorted, l, index - 1);
    sorted = _quick_sort_with_new_vector(&sorted, index + 1, r);

    sorted
}
