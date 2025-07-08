use proconio::{input, marker::Chars};

const ROCK: usize = 0;
const PAPER: usize = 1;
const SCISSORS: usize = 2;

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    // dp[N戦目が終わったとき][最後に出したのがR=0, P=1, S=2] = 最大勝利数
    let mut dp = vec![vec![0; 3]; n + 1];

    for i in 1..=n {
        let dist = s[i - 1];

        match dist {
            'R' => {
                dp[i][PAPER] = dp[i - 1][ROCK].max(dp[i - 1][SCISSORS]) + 1;
                dp[i][ROCK] = dp[i - 1][SCISSORS].max(dp[i - 1][PAPER]);
                dp[i][SCISSORS] = 0;
            }
            'P' => {
                dp[i][SCISSORS] = dp[i - 1][ROCK].max(dp[i - 1][PAPER]) + 1;
                dp[i][PAPER] = dp[i - 1][ROCK].max(dp[i - 1][SCISSORS]);
                dp[i][ROCK] = 0;
            }
            'S' => {
                dp[i][ROCK] = dp[i - 1][PAPER].max(dp[i - 1][SCISSORS]) + 1;
                dp[i][SCISSORS] = dp[i - 1][PAPER].max(dp[i - 1][ROCK]);
                dp[i][PAPER] = 0;
            }
            _ => unreachable!(),
        }
    }

    println!("{}", dp[n][ROCK].max(dp[n][PAPER]).max(dp[n][SCISSORS]));
}
