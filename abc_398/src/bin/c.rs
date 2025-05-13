use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let maps = value_map(&a);
    let unique = maps
        .iter()
        .filter(|(_, v)| v.len() == 1)
        .collect::<Vec<_>>();

    let max_item = unique.iter().max_by_key(|(k, _)| k);
    let max_person = max_item.and_then(|(_, v)| v.iter().max());

    if let Some(ans) = max_person {
        println!("{ans}");
    } else {
        println!("-1");
    }
}

fn value_map(arr: &[usize]) -> HashMap<usize, Vec<usize>> {
    let mut counts = HashMap::new();
    for (i, &x) in arr.iter().enumerate() {
        counts.entry(x).or_insert(vec![]).push(i + 1);
    }
    counts
}
