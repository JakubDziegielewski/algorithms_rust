
fn main() {
    let mut a: &mut [i32] = &mut [0, 1, -2, 5500, 10, 11, 0, -10, 100, -100, -99, 100, 20202, -3];
    let len = a.len();
    quick_sort(&mut a,  0, len as i32 - 1);
    let mut b = [1, 0, -2, 5500, 10, 11, 0, -10, 100, -100, -99, 100, -3, 20202];
    b.sort();
    assert_eq!(a, b);

    println!("{:?}", a)
}
fn quick_sort(mut a: &mut [i32], l: i32, r: i32) {

    if l>=r{
        return;
    }

    let mut index = l;

    for i in l+1..r+1{
        if a[i as usize]<=a[l as usize]{
            index = index + 1;
            (a[i as usize], a[index as usize]) = (a[index as usize], a[i as usize]);
        }
    }

    (a[l as usize], a[index as usize]) = (a[index as usize], a[l as usize]);
    
    quick_sort(&mut a, l, index -1);
    quick_sort(&mut a, index + 1, r);
    
}

/*fn quick_sort(a: &[i32], l: i32, r: i32) -> Vec<i32> {
    let mut sorted = Vec::from(a);

    if l>=r{
        return sorted;
    }

    let mut index = l;

    for i in l+1..r+1{
        if sorted[i as usize]<=sorted[l as usize]{
            index = index + 1;
            (sorted[i as usize], sorted[index as usize]) = (sorted[index as usize], sorted[i as usize]);
        }
    }

    (sorted[l as usize], sorted[index as usize]) = (sorted[index as usize], sorted[l as usize]);
    
    sorted = quick_sort(&sorted, l, index -1);
    sorted = quick_sort(&sorted, index + 1, r);
    
    sorted
}
 */