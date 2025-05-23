use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut left_one = 0;
    let mut zero_positions = Vec::new();

    for c in s {
        match c {
            '1' => left_one += 1,
            '0' => zero_positions.push(left_one),
            _ => {}
        }
    }

    let number_of_one = n - zero_positions.len();
    let ans = zero_positions
        .iter()
        .map(|&p| p.min(number_of_one - p))
        .sum::<usize>();

    println!("{ans}");
}
