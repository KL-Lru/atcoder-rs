use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        x: [usize; m],
        a: [usize; m],
    }

    let mut field = HashMap::new();
    for i in 0..m {
        *field.entry(x[i]).or_insert(0) += a[i];
    }
    field.entry(n).or_insert(0);

    let mut keys = field.keys().cloned().collect::<Vec<_>>();
    keys.sort();

    let mut buf = 0;
    let mut ans = 0;

    if keys[0] != 1 {
        println!("-1");
        return;
    }
    buf += field[&keys[0]] - 1;

    for i in 1..keys.len() {
        let diff = keys[i] - keys[i - 1] - 1;
        if buf < diff {
            println!("-1");
            return;
        }

        ans += sum_to_n(buf) - sum_to_n(buf - diff);

        buf -= diff;
        ans += buf;
        if field[&keys[i]] == 0 && buf == 0 {
            println!("-1");
            return;
        }

        buf += field[&keys[i]];
        buf -= 1;
    }

    if buf != 0 {
        println!("-1");
    } else {
        println!("{ans}");
    }
}

fn sum_to_n(n: usize) -> usize {
    n * (n + 1) / 2
}
