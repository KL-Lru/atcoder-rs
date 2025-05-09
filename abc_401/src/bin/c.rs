use proconio::input;
const MOD: usize = 1e+9 as usize;

fn main() {
    input! {
        n: usize, k: usize,
    }

    if n < k {
        println!("1");
        return;
    }

    let mut dp = vec![0; n + 1];

    dp.iter_mut().take(k).for_each(|dpi| *dpi = 1);
    dp[k] = k;
    for i in (k + 1)..=n {
        dp[i] = (dp[i - 1] * 2 - dp[i - k - 1] + MOD) % MOD;
    }

    println!("{}", dp[n]);
}
