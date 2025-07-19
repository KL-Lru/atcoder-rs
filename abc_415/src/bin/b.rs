use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let idx = s
        .into_iter()
        .enumerate()
        .filter_map(|(i, c)| match c {
            '#' => Some((i + 1).to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();

    for v in idx.chunks(2) {
        println!("{}", v.join(","));
    }
}
