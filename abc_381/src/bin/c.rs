use proconio::{input, marker::Chars};

fn main() {
    input! {
      _: usize,
      s: Chars,
    }

    let rl = run_length(&s);

    let mut ans = 0;

    for i in 0..rl.len() {
        if rl[i].0 == '/' {
            ans = ans.max(1);

            let has_lr = i > 0 && i < rl.len() - 1;
            let single_slash = rl[i].1 == 1;
            let left_one = has_lr && rl[i - 1].0 == '1';
            let right_two = has_lr && rl[i + 1].0 == '2';

            if single_slash && left_one && right_two {
                ans = ans.max(2 * (rl[i - 1].1.min(rl[i + 1].1)) + 1);
            }
        }
    }

    println!("{ans}");
}

fn run_length(s: &[char]) -> Vec<(char, usize)> {
    s.iter().fold(Vec::new(), |mut acc, si| {
        if let Some((last_char, count)) = acc.last_mut() {
            if last_char == si {
                *count += 1;
            } else {
                acc.push((*si, 1));
            }
        } else {
            acc.push((*si, 1));
        }
        acc
    })
}
