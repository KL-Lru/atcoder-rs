use proconio::input;

fn main() {
    input! {
        x: usize,
    }

    for i in 1.. {
        let factorial_i = factorial(i);
        if factorial_i == x {
            println!("{i}");
            return;
        }
    }
}

fn factorial(n: usize) -> usize {
    if n == 0 || n == 1 {
        return 1;
    }
    (2..=n).product()
}
