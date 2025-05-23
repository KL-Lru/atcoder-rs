use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
        q: [usize; n],
    }

    let mut idx_to_watching_map = HashMap::new();
    let mut bib_to_idx_map = HashMap::new();
    let mut idx_to_bib_map = HashMap::new();

    for (idx, &pi) in p.iter().enumerate() {
        idx_to_watching_map.entry(idx + 1).or_insert(pi);
    }

    for (idx, &qi) in q.iter().enumerate() {
        bib_to_idx_map.entry(qi).or_insert(idx + 1);
        idx_to_bib_map.entry(idx + 1).or_insert(qi);
    }

    for bib in 1..=n {
        let idx = bib_to_idx_map[&bib];
        let watching = idx_to_watching_map[&idx];
        let ans = idx_to_bib_map[&watching];

        print!("{ans}");
        if bib == n {
            println!()
        } else {
            print!(" ")
        }
    }
}
