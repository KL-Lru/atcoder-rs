use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
    }

    if a.iter().sum::<usize>() <= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
