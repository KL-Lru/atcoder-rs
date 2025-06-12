use proconio::{input, marker::Chars};

fn main() {
    input! {
      _: usize, k: usize,
      s: Chars,
    }

    let mut rl = run_length(&s);
    let kth_idx = find_kth_one(&rl, k);

    rl.swap(kth_idx, kth_idx - 1);
    let mut ans = String::new();
    for (c, count) in rl {
        ans.push_str(&c.to_string().repeat(count));
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

fn find_kth_one(rl: &[(char, usize)], k: usize) -> usize {
    let mut one_count = 0;
    for (i, (c, _)) in rl.iter().enumerate() {
        if *c == '1' {
            one_count += 1;
            if one_count == k {
                return i;
            }
        }
    }

    unreachable!()
}
