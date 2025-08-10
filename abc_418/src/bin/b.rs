use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let pos_of_t = s
        .iter()
        .enumerate()
        .filter(|(_, char)| *char == &'t')
        .map(|(v, _)| v)
        .collect::<Vec<_>>();

    if pos_of_t.len() < 3 {
        println!("0");
    } else {
        let mut ans = 0.0f64;
        let t_count = pos_of_t.len();

        for fi in 0..t_count {
            for li in (fi + 1)..t_count {
                ans = ans.max(calc(fi, li, &pos_of_t));
            }
        }

        println!("{:.10}", ans);
    }
}

fn calc(fi: usize, li: usize, pos: &[usize]) -> f64 {
    let len = pos[li] - pos[fi] + 1;
    let t_count = li - fi + 1;

    (t_count as f64 - 2.0) / (len as f64 - 2.0)
}
