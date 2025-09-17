use proconio::input;

fn main() {
    input! {
        n: usize,
        l: [usize; n]
    }

    let mut a = 0;
    let mut b = n - 1;

    while a < n - 1 && l[a] == 0 {
        a += 1;
    }

    while b > 0 && l[b] == 0 {
        b -= 1;
    }

    if a >= b {
        println!("0");
    } else {
        println!("{}", b - a);
    }
}
