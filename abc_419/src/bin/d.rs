use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, m: usize,
        mut s: Chars,
        mut t: Chars,
        swap: [(usize, usize); m]
    }

    let mut p = vec![0; n + 1];

    for (l, r) in swap {
        p[l - 1] += 1;
        p[r] -= 1;
    }

    let p = p.iter().fold(vec![], |mut acc, &x| {
        match acc.last() {
            Some(a) => acc.push(a + x),
            None => acc.push(x),
        }

        acc
    });

    let mut ans = vec![];

    for (idx, pi) in p.iter().take(n).enumerate() {
        if pi % 2 == 0 {
            ans.push(s[idx]);
        } else {
            ans.push(t[idx]);
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
