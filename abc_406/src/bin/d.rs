use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        h: usize, w: usize, n: usize,
    }

    let mut px = vec![vec![]; h + 1];
    let mut py = vec![vec![]; w + 1];

    for i in 0..n {
        input! { x: usize, y: usize }
        px[x].push(i);
        py[y].push(i);
    }

    let mut ux = vec![false; h + 1];
    let mut uy = vec![false; w + 1];
    let mut used = vec![false; n];

    input! { q: usize };
    for _ in 0..q {
        input! { t: usize, z: usize }

        match t {
            1 if ux[z] => println!("0"),
            2 if uy[z] => println!("0"),
            1 => {
                println!("{}", px[z].iter().filter(|&&i| !used[i]).count());

                px[z].iter().for_each(|&i| used[i] = true);
                ux[z] = true;
            }
            2 => {
                println!("{}", py[z].iter().filter(|&&i| !used[i]).count());

                py[z].iter().for_each(|&i| used[i] = true);
                uy[z] = true;
            }
            _ => unreachable!(),
        }
    }
}
