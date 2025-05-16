use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    if has_same_three(&a) {
        println!("Yes")
    } else {
        println!("No")
    }
}

fn has_same_three(a: &[usize]) -> bool {
    for i in 0..(a.len() - 2) {
        if a[i] == a[i + 1] && a[i] == a[i + 2] {
            return true;
        }
    }

    false
}
