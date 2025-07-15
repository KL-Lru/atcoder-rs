use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    match s.as_slice() {
        ['R', 'M', _] | ['R', _, 'M'] | [_, 'R', 'M'] => {
            println!("Yes");
        }
        _ => {
            println!("No");
        }
    }
}
