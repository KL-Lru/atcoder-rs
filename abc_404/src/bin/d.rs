use std::collections::{HashMap, HashSet};

use proconio::input;

const INF: u64 = 1e+15 as u64;

fn main() {
    input! {
      n: usize,
      m: usize,
      c: [u64; n],
      a: [[usize]; m]
    }

    let animal_map = animal_map(&a);

    let perm_max = 3_usize.pow(n as u32);
    let mut ans = INF;

    for perm in 0..perm_max {
        let perm_vec = perm_vec(n, perm);
        if is_satisfy(m, &perm_vec, &animal_map) {
            ans = ans.min(sum_cost(&c, &perm_vec));
        }
    }

    println!("{ans}");
}

fn animal_map(a: &Vec<Vec<usize>>) -> HashMap<usize, HashSet<usize>> {
    let mut map = HashMap::<usize, HashSet<usize>>::new();

    for i in 0..a.len() {
        for j in 0..(a[i].len()) {
            map.entry(a[i][j] - 1).or_default().insert(i);
        }
    }

    map
}

fn perm_vec(n: usize, perm: usize) -> Vec<usize> {
    let mut vec = vec![0; n];
    let mut p = perm;
    for i in 0..n {
        vec[i] = p % 3;
        p /= 3;
    }

    vec
}

fn is_satisfy(
    m: usize,
    perm_vec: &Vec<usize>,
    animal_map: &HashMap<usize, HashSet<usize>>,
) -> bool {
    let mut animal_count = HashMap::<usize, usize>::new();

    for (idx, count) in perm_vec.iter().enumerate() {
        if let Some(animal) = animal_map.get(&idx) {
            for a in animal {
                *animal_count.entry(*a).or_insert(0) += count;
            }
        }
    }

    animal_count.len() == m && animal_count.values().all(|count| *count >= 2)
}

fn sum_cost(c: &Vec<u64>, perm_vec: &Vec<usize>) -> u64 {
    let mut sum = 0;
    for (idx, count) in perm_vec.iter().enumerate() {
        sum += c[idx] * (*count as u64);
    }
    sum
}
