use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut ti = 0;

    for si in s {
        if si.to_uppercase().to_string() == t[ti].to_string() {
            ti += 1;
            if ti >= t.len() {
                break;
            }
        }
    }

    if ti >= t.len() || (ti == t.len() - 1 && t.last() == Some(&'X')) {
        println!("Yes");
    } else {
        println!("No");
    }
}
