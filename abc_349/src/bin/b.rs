use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut counts = vec![0; 'z' as usize - 'a' as usize + 1];

    for c in 'a'..='z' {
        counts[c as usize - 'a' as usize] = s.iter().filter(|&&x| x == c).count();
    }

    for i in 1..=100 {
        let alpha = counts.iter().filter(|&&x| x == i).count();
        if alpha != 0 && alpha != 2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
