use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: String,
    }

    for w in 1..s.len() {
        for c in 0..w {
            let sv = s.chunks(w).collect::<Vec<_>>();
            let pick_s = sv
                .iter()
                .filter_map(|chunk| chunk.get(c))
                .collect::<String>();

            if pick_s == t {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
