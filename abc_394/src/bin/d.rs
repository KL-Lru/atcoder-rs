use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars
    }

    let mut buf = Vec::new();

    while !s.is_empty() {
        match (s.last(), buf.last()) {
            (Some('('), Some(')')) | (Some('['), Some(']')) | (Some('<'), Some('>')) => {
                s.pop();
                buf.pop();
            }
            (Some(&c), _) => {
                s.pop();
                buf.push(c);
            }
            (None, _) => break,
        }
    }

    if buf.is_empty() {
        println!("Yes");
    } else {
        println!("No");
    }
}
