use proconio::input;

fn main() {
    input! {
        a: (isize, isize),
        b: (isize, isize),
        c: (isize, isize),
    }

    let ab = vector(a, b);
    let ac = vector(a, c);
    let bc = vector(b, c);

    if inner_product(&ab, &bc) == 0 || inner_product(&ab, &ac) == 0 || inner_product(&ac, &bc) == 0
    {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn vector(pa: (isize, isize), pb: (isize, isize)) -> (isize, isize) {
    (pb.0 - pa.0, pb.1 - pa.1)
}

fn inner_product(va: &(isize, isize), vb: &(isize, isize)) -> isize {
    va.0 * vb.0 + va.1 * vb.1
}
