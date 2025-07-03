use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut count = 0;

    while a.iter().filter(|&&x| x > 0).count() > 1 {
        a.sort_by(|a, b| b.cmp(a));

        a[0] -= 1;
        a[1] -= 1;
        count += 1;
    }

    println!("{count}");
}
