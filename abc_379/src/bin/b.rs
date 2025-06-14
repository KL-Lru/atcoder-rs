use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize, k: usize,
        s: Chars,
    }

    let rl = run_length(&s);

    println!(
        "{}",
        rl.iter()
            .filter(|(c, _)| *c == 'O')
            .map(|(_, count)| count / k)
            .sum::<usize>()
    );
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
