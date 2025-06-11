use proconio::{input, marker::Chars};

fn main() {
    input! {
      n: usize,
      s: Chars,
    }

    let rl = run_length(&s);

    if s == ['/'] || rl == [('1', n / 2), ('/', 1), ('2', n / 2)] {
        println!("Yes");
    } else {
        println!("No");
    }
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
