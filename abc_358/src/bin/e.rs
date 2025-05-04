use std::cmp::min;

use proconio::input;

const ALPHABET_COUNT: usize = 26;
const MOD: u64 = 998244353;

fn factorial_cache() -> Vec<u64> {
    let mut cache = vec![0u64; 1002];
    cache[0] = 1;

    for n in 1..=1001 {
        cache[n] = mod_prod(cache[n - 1], n as u64);
    }

    cache
}

fn factorial_inverse_cache(factorial: &Vec<u64>) -> Vec<u64> {
    let mut cache = vec![0u64; 1002];
    cache[0] = 1;

    for n in 1..=1001 {
        cache[n] = mod_inverse(*factorial.get(n).unwrap());
    }

    cache
}

fn mod_inverse(a: u64) -> u64 {
    let mut x = 1;
    let mut ta = a;
    let mut n = MOD - 2;

    while n != 0 {
        if n % 2 == 1 {
            x = (x * ta) % MOD;
            n -= 1;
        } else {
            ta = (ta * ta) % MOD;
            n /= 2;
        }
    }

    x
}

fn mod_prod(x: u64, y: u64) -> u64 {
    (x * y) % MOD
}

fn mod_comb(n: u64, r: u64, factorial_cache: &Vec<u64>, inverse_cache: &Vec<u64>) -> u64 {
    mod_prod(
        *factorial_cache.get(n as usize).unwrap(),
        mod_prod(
            *inverse_cache.get(r as usize).unwrap(),
            *inverse_cache.get((n - r) as usize).unwrap(),
        ),
    )
}

fn main() {
    input! {
      k: usize,
      c: [usize; ALPHABET_COUNT],
    }

    let factorial = factorial_cache();
    let factorial_inverse = factorial_inverse_cache(&factorial);

    // dp[number of alphabet kind][length of string] = number of patterns
    let mut dp = vec![vec![0u64; k + 1]; ALPHABET_COUNT + 1];

    // if use only 'A', pattern is only 1
    for str_len in 0..=min(c[0], k) {
        dp[1][str_len] = 1;
    }

    for kind_num in 2..=ALPHABET_COUNT {
        for str_len in 0..=k {
            for using_char in 0..=min(c[kind_num - 1], str_len) {
                let remain = dp[kind_num - 1][str_len - using_char];
                let combination = mod_comb(
                    str_len as u64,
                    using_char as u64,
                    &factorial,
                    &factorial_inverse,
                );
                dp[kind_num][str_len] += mod_prod(remain, combination);
                dp[kind_num][str_len] %= MOD;
            }
        }
    }

    let mut ans = 0;
    for str_len in 1..=k {
        ans += dp[ALPHABET_COUNT][str_len];
    }

    println!("{:?}", ans % MOD);
}
