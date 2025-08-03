use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        mut a: [usize; n],
        b: [usize; m],
    }

    for bi in b {
        let p = a
            .iter()
            .enumerate()
            .find_map(|(idx, &ai)| if ai == bi { Some(idx) } else { None });
        if let Some(pos) = p {
            a.remove(pos);
        }
    }

    println!(
        "{}",
        a.iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}
