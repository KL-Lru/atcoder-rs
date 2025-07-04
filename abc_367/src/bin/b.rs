use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut x: Chars,
    }

    for &c in x.clone().iter().rev() {
        match c {
            '0' => {
                x.pop();
            }
            '.' => {
                x.pop();
                break;
            }
            _ => break,
        }
    }

    println!("{}", x.iter().collect::<String>());
}
