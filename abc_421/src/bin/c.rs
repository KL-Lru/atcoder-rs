use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    println!(
        "{}",
        distance(&s, &repeat_chars("AB", n)).min(distance(&s, &repeat_chars("BA", n)))
    )
}

fn distance(s: &[char], t: &[char]) -> usize {
    let sp = a_positions(s);
    let tp = a_positions(t);

    sp.into_iter()
        .zip(tp)
        .fold(0, |acc, (spi, tpi)| acc + spi.abs_diff(tpi))
}

fn repeat_chars(s: &str, n: usize) -> Vec<char> {
    s.repeat(n).chars().collect()
}

fn a_positions(s: &[char]) -> Vec<usize> {
    s.iter()
        .enumerate()
        .filter_map(|(idx, &c)| if c == 'A' { Some(idx) } else { None })
        .collect()
}
