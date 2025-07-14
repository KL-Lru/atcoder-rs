use proconio::input;

fn main() {
    input! {
        n: usize, k: usize, x: usize,
        mut a: [usize; n]
    }

    a.insert(k, x);

    println!(
        "{}",
        a.iter()
            .map(usize::to_string)
            .collect::<Vec<String>>()
            .join(" ")
    )
}
