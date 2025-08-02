use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n]
    }

    let ans = a
        .iter()
        .filter(|&&ai| ai % k == 0)
        .map(|ai| (ai / k).to_string())
        .collect::<Vec<_>>();

    println!("{}", ans.join(" "))
}
