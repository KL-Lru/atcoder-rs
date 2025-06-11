use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
      s: Chars,
    }

    let rl = run_length(&s);

    let all_two_run = rl.iter().all(|(_, count)| *count == 2);
    let all_group_unique = rl.len() == HashSet::<char>::from_iter(rl.iter().map(|(c, _)| *c)).len();

    if all_two_run && all_group_unique {
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
