use proconio::{input, marker::Chars};

fn main() {
    input! { n: Chars }

    let one = count('1', &n);
    let two = count('2', &n);
    let three = count('3', &n);

    if one == 1 && two == 2 && three == 3 {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn count(c: char, n: &[char]) -> usize {
    n.iter().filter(|&&x| x == c).count()
}
