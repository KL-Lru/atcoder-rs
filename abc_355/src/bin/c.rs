use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize, t: usize,
        a: [usize; t],
    }

    let mut map = HashMap::new();
    for &ai in a.iter() {
        map.insert(ai, true);
    }

    let mut satisfied = [
        horizontal_bingo(n, &map),
        vertical_bingo(n, &map),
        cross_bingo(n, &map),
    ]
    .concat();

    if satisfied.is_empty() {
        println!("-1");
        return;
    }

    for (x, ai) in a.into_iter().rev().enumerate() {
        satisfied.retain(|v| !v.contains(&ai));

        if satisfied.is_empty() {
            println!("{}", t - x);
            return;
        }
    }
}

fn cell_num(n: usize, x: usize, y: usize) -> usize {
    (n * (x - 1)) + y
}

fn horizontal_bingo(n: usize, map: &HashMap<usize, bool>) -> Vec<HashSet<usize>> {
    let mut result = vec![];

    for x in 1..=n {
        let mut set = HashSet::new();
        let mut fulfilled = true;

        for y in 1..=n {
            if map.get(&cell_num(n, x, y)).is_none() {
                fulfilled = false;
                break;
            }

            set.insert(cell_num(n, x, y));
        }

        if fulfilled {
            result.push(set);
        }
    }

    result
}

fn vertical_bingo(n: usize, map: &HashMap<usize, bool>) -> Vec<HashSet<usize>> {
    let mut result = vec![];

    for y in 1..=n {
        let mut set = HashSet::new();
        let mut fulfilled = true;

        for x in 1..=n {
            if map.get(&cell_num(n, x, y)).is_none() {
                fulfilled = false;
                break;
            }

            set.insert(cell_num(n, x, y));
        }

        if fulfilled {
            result.push(set);
        }
    }

    result
}

fn cross_bingo(n: usize, map: &HashMap<usize, bool>) -> Vec<HashSet<usize>> {
    let mut result = vec![];

    let mut set = HashSet::new();
    let mut fulfilled = true;

    for (x, y) in (1..=n).zip(1..=n) {
        if map.get(&cell_num(n, x, y)).is_none() {
            fulfilled = false;
            break;
        }

        set.insert(cell_num(n, x, y));
    }

    if fulfilled {
        result.push(set);
    }

    let mut set = HashSet::new();
    let mut fulfilled = true;

    for (x, y) in (1..=n).zip((1..=n).rev()) {
        if map.get(&cell_num(n, x, y)).is_none() {
            fulfilled = false;
            break;
        }

        set.insert(cell_num(n, x, y));
    }

    if fulfilled {
        result.push(set);
    }

    result
}
