use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    // dp[i 体目][i体目を含めて倒した数の合計が偶数(0) or 奇数(1)] = 最大経験値
    let mut dp = vec![vec![0; 2]; n + 1];

    // 1体目は奇数にしかならない
    dp[1][1] = a[0];

    for i in 2..=n {
        let x = a[i - 1];

        // i 体目を倒さない or 倒す
        dp[i][0] = dp[i - 1][0].max(dp[i - 1][1] + 2 * x);
        dp[i][1] = dp[i - 1][1].max(dp[i - 1][0] + x);
    }

    println!("{}", dp[n][0].max(dp[n][1]));
}
