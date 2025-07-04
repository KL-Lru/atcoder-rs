use proconio::input;

fn main() {
    input! {
        mut a: usize, b: usize, mut c: usize
    }

    if b > c {
        c += 24;
        if a < b {
            a += 24;
        }
    }

    if b < a && a < c {
        println!("No");
    } else {
        println!("Yes");
    }
}
