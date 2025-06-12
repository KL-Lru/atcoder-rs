use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    let mut ans = vec![];
    let mut current = 0;

    for si in s {
        match si {
            '|' if current != 0 => {
                ans.push(current);
                current = 0;
            }
            '-' => {
                current += 1;
            }
            _ => {}
        }
    }

    for ai in ans {
        print!("{} ", ai);
    }
    println!();
}
