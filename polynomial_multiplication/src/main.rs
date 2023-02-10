fn main() {
    let len: usize = 10;
    let a: &[i32] = &[1, -11, -121, 20, -4, 1, 8, 34, 11, 1];
    let b: &[i32] = &[9, 3, 2, 112, 21, 7, 1, 1, 10, 1];

    let c = devide_and_conquer_method(&a[0..len], &b[0..len]);
    //println!("{:?}", c);

    let d = naive_devide_and_conquer_method(&a[0..len], &b[0..len]);
    //println!("{:?}", d);

    let e = naive_method(&a[0..len], &b[0..len]);
    //println!("{:?}", e);

    assert_eq!(c, d);
    assert_eq!(c, e);
}

//naive method iterating over both polynomials and calculating each coefficient: O(n^2)
fn naive_method(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut result = vec![0; 2 * a.len() - 1];
    for (i, item_a) in a.iter().enumerate() {
        for (j, item_b) in b.iter().enumerate() {
            result[i + j] = result[i + j] + item_a * item_b;
        }
    }
    result
}

//method using devide and conquer design; not the most efficient one: O(n^2)
fn naive_devide_and_conquer_method(a: &[i32], b: &[i32]) -> Vec<i32> {
    let n = a.len();
    let mut result: Vec<i32> = Vec::new();

    if n == 1 {
        result.push(a[0] * b[0]);
        return result;
    }

    //divide polynomials into halves
    let mut a1 = Vec::from(&a[..n / 2]);
    let a0 = Vec::from(&a[n / 2..]);
    let mut b1 = Vec::from(&b[..n / 2]);
    let b0 = Vec::from(&b[n / 2..]);

    //calculate a1b1 and a0b0
    let temp_a = naive_devide_and_conquer_method(&a1, &b1);
    let mut temp_b = naive_devide_and_conquer_method(&a0, &b0);

    //begin constructing the result
    result = temp_a;
    result.push(0);
    result.append(&mut temp_b);

    /*  if n is not even,
        a1.len() + 1 = a0.len() and b1.len() + 1 = b0.len()
        and its neccessary that corresponding vectors are of the same size
        so we need to pad them with 0s
    */
    if n % 2 == 1 {
        a1.push(0);
        b1.push(0);
    }

    //calulate a1b0 and a0b1, then add them to the result
    let temp_c = naive_devide_and_conquer_method(&a1, &b0);
    let temp_d = naive_devide_and_conquer_method(&a0, &b1);

    for i in 0..temp_c.len() {
        let ind = (n / 2) + i;
        result[ind] = result[ind] + temp_c[i] + temp_d[i];
    }
    result
}

//method using devide and conquer design: O(n^log_2(3))
fn devide_and_conquer_method(a: &[i32], b: &[i32]) -> Vec<i32> {
    let n = a.len();
    let mut result: Vec<i32> = Vec::new();

    //if polymonials have one coefficient each, return ther product
    if n == 1 {
        result.push(a[0] * b[0]);
        return result;
    }

    //divide polynomials into halves
    let mut a1 = Vec::from(&a[..n / 2]);
    let a0 = Vec::from(&a[n / 2..]);
    let mut b1 = Vec::from(&b[..n / 2]);
    let b0 = Vec::from(&b[n / 2..]);

    //calculate a1b1 and a0b0
    let mut temp_a = devide_and_conquer_method(&a1, &b1);
    let temp_b = devide_and_conquer_method(&a0, &b0);

    //begin constructing the result
    result = temp_a.clone(); //clone because tema_a is needed later; don't take ownership
    result.push(0); //pad with 0 because temp_a.len + temp_b.len() = 2n - 2
    result.append(&mut temp_b.clone()); //append temp_b.clone(); don't take ownership

    /*  if n is not even, temp_a.len() + 2 = temp_b.len(),
        as a1.len() + 1 = a0.len() and b1.len() + 1 = b0.len()
        and its neccessary that corresponding vectors are of the same size
        so we need to pad them with 0s
    */
    if n % 2 == 1 {
        temp_a.append(&mut vec![0; 2]);
        a1.push(0);
        b1.push(0);
    }

    //calculate a1+a0 and b1+b0
    let mut sum_a: Vec<i32> = Vec::new();
    let mut sum_b: Vec<i32> = Vec::new();

    for i in 0..a1.len() {
        sum_a.push(a1[i] + a0[i]);
        sum_b.push(b1[i] + b0[i]);
    }

    //calculate (a1+a0)(b1+b0)
    let mut temp_c: Vec<i32> = devide_and_conquer_method(&sum_a, &sum_b);

    //calculete (a1+a0)(b1+b0) - a1b1 - a0b0 and add to the result
    for i in 0..temp_c.len() {
        temp_c[i] = temp_c[i] - temp_a[i] - temp_b[i];
        let ind = n / 2 + i;
        result[ind] = result[ind] + temp_c[i];
    }
    result
}
