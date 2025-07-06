use proconio::{input, marker::Chars};

const NOTHING: char = '*';

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let m = s.iter().map(|si| si.len()).max().unwrap_or(1);
    for i in 0..m {
        let mut t = s
            .iter()
            .rev()
            .map(|si| si.get(i).unwrap_or(&NOTHING))
            .collect::<String>();

        while t.ends_with(NOTHING) {
            t.pop();
        }

        println!("{t}");
    }
}
