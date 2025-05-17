use proconio::input;

const BLACK: char = '#';
const WHITE: char = '.';

fn main() {
    input! { n: usize }

    let base = n / 2 + n % 2;

    let mut ans = Vec::new();

    for i in 0..base {
        ans.push(flipped_str(base, i));
    }

    ans = ans
        .iter()
        .map(|s| palindrome(s.chars().collect(), n).iter().collect())
        .collect();

    ans = palindrome(ans, n);

    println!("{}", ans.join("\n"));
}

fn flipped_str(n: usize, flip: usize) -> String {
    let mut s = String::new();
    let mut c = BLACK;

    for i in 0..n {
        if i <= flip {
            if i % 2 == 0 {
                c = BLACK;
            } else {
                c = WHITE;
            }
            s.push(c);
        } else {
            s.push(c);
        }
    }

    s
}

fn palindrome<T: Clone>(t: Vec<T>, n: usize) -> Vec<T> {
    let base = n / 2 + n % 2;
    let mut pre = t[0..base].to_vec();
    let suf = t[0..(n / 2)].iter().rev().cloned().collect::<Vec<_>>();

    pre.extend(suf);
    pre
}
