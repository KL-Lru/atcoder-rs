use proconio::input;

fn main() {
    input! {
        a: [usize; 9],
        b: [usize; 8],
    }

    println!("{}", a.iter().sum::<usize>() - b.iter().sum::<usize>() + 1);
}
