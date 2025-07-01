use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = vec![];
    for i in 1..=n {
        input! { ai: [usize; i] }
        a.push(ai.iter().map(|&x| x - 1).collect::<Vec<_>>());
    }

    let mut ans = 0;
    for i in 0..n {
        ans = a[ans.max(i)][ans.min(i)];
    }

    println!("{}", ans + 1);
}
