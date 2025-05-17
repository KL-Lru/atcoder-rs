use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    if is_increasing(&a) {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn is_increasing(a: &[usize]) -> bool {
    for i in 1..a.len() {
        if a[i] <= a[i - 1] {
            return false;
        }
    }
    true
}
