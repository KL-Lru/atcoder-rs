use std::collections::HashMap;

const INF: usize = 1e+7 as usize;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut pos_map = HashMap::<usize, Vec<usize>>::new();

    for (idx, ai) in a.iter().enumerate() {
        pos_map.entry(*ai).or_default().push(idx);
    }

    let ans = pos_map.iter().fold(INF, |acc, (_, values)| {
        let mut current = acc;
        for i in 0..(values.len() - 1) {
            current = current.min(values[i + 1] - values[i] + 1);
        }

        current
    });

    if ans == INF {
        println!("-1");
    } else {
        println!("{ans}");
    }
}
