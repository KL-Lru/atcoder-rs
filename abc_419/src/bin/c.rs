use proconio::input;

fn main() {
    input! {
        n: usize,
        points: [(usize, usize); n],
    }

    let x = max_distance(points.iter().map(|p| p.0).collect());
    let y = max_distance(points.iter().map(|p| p.1).collect());

    println!("{}", x.max(y));
}

fn max_distance(p: Vec<usize>) -> usize {
    if let (Some(mx), Some(mn)) = (p.iter().max(), p.iter().min()) {
        let mid = (mx + mn) / 2;

        mx.abs_diff(mid).max(mn.abs_diff(mid))
    } else {
        unreachable!()
    }
}
