use proconio::input;

fn main() {
    input! {
          n: usize, a: usize, b: usize,
          mut d: [usize; n],
    }

    let md = a + b;

    d = d.iter().map(|&di| di % md).collect();
    let dist = d
        .iter()
        .map(|&di| min_dist(di, d[0], md))
        .collect::<Vec<isize>>();
    let r = dist.iter().max().expect("Not Empty") - dist.iter().min().expect("Not Empty") + 1;

    if r <= a as isize {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn min_dist(v: usize, base: usize, md: usize) -> isize {
    let omd = ((v + md) - base) as isize;
    let sd = (v as isize) - (base as isize);

    if omd.abs() < sd.abs() {
        omd
    } else {
        sd
    }
}
