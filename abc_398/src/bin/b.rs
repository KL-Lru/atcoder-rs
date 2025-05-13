use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        a: [usize; 7]
    }

    let counts = count_map(&a);
    if count_over(&counts, 3) >= 1 && count_over(&counts, 2) >= 2 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn count_map(arr: &[usize]) -> HashMap<usize, usize> {
    let mut counts = HashMap::new();
    for &x in arr {
        *counts.entry(x).or_insert(0) += 1;
    }
    counts
}

fn count_over(counts: &HashMap<usize, usize>, threshold: usize) -> usize {
    counts.values().filter(|&&count| count >= threshold).count()
}
