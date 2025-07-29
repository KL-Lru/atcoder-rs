use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut flip = true;
    let mut ans = vec![];

    for c in s {
        match c {
            '#' => {
                flip = true;
                ans.push('#');
            }
            '.' if flip => {
                flip = false;
                ans.push('o');
            }
            '.' if !flip => {
                ans.push('.');
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
