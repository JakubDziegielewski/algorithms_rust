fn main() {
    let a: &mut [i32] = &mut [4, 4, 3, 3, 2, 1];
    let len = a.len();
    let b: &mut Vec<i32> = &mut vec![0; len];
    b.clone_from_slice(&a[..]);
    quick_sort3(&mut a[..], 0, len as i32 - 1);
    quick_sort3_with_shorter_slicec(&mut b[..]);
    assert_eq!(a, b);
    println!("{:?}", b);
}

fn quick_sort3(mut a: &mut [i32], l: i32, r: i32) {
    //clasic method
    if l >= r {
        return;
    }

    let mut j = l;
    let mut k = l;

    for i in l + 1..r + 1 {
        if a[i as usize] < a[l as usize] {
            j = j + 1;
            k = k + 1;
            (a[i as usize], a[k as usize]) = (a[k as usize], a[i as usize]);
            (a[j as usize], a[k as usize]) = (a[k as usize], a[j as usize]);
    
        } else if a[i as usize] == a[l as usize] {
            k = k + 1;
            (a[i as usize], a[k as usize]) = (a[k as usize], a[i as usize])
        }
    }

    (a[l as usize], a[j as usize]) = (a[j as usize], a[l as usize]);

    quick_sort3(&mut a, l, j - 1);
    quick_sort3(&mut a, j + 1, r);
}
fn quick_sort3_with_shorter_slicec(a: &mut [i32]) {
    let len = a.len();
    if len <= 0 {
        return;
    }
    //recursive methods are only given the slice, that they are supposed to sort

    let mut j = 0;
    let mut k = 0;
    for i in 1..len {
        if a[i as usize] < a[0] {
            j = j + 1;
            k = k + 1;
            (a[i as usize], a[k as usize]) = (a[k as usize], a[i as usize]);
            (a[j as usize], a[k as usize]) = (a[k as usize], a[j as usize]);
        } else if a[i as usize] == a[0] {
            k = k + 1;
            (a[i as usize], a[k as usize]) = (a[k as usize], a[i as usize]);
        }
    }
    (a[0], a[j as usize]) = (a[j as usize], a[0]);

    quick_sort3_with_shorter_slicec(&mut a[0..j as usize]);
    quick_sort3_with_shorter_slicec(&mut a[j as usize + 1..len]);
}
