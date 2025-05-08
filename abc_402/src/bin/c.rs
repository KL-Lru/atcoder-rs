use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        ka: [[usize]; m],
        b: [usize; n],
    }

    let mut recipe_count = Vec::from_iter(ka.iter().map(|k| k.len()));
    let mut food_map = HashMap::<usize, Vec<usize>>::new();
    for (i, k) in ka.iter().enumerate() {
        for &ki in k.iter() {
            food_map.entry(ki).or_default().push(i);
        }
    }

    let mut ans = 0;
    for &bi in b.iter() {
        let recipes = food_map.entry(bi).or_default();
        for &recipe in recipes.iter() {
            recipe_count[recipe] -= 1;
            if recipe_count[recipe] == 0 {
                ans += 1;
            }
        }

        println!("{ans}");
    }
}
