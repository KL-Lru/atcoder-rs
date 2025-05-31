use proconio::input;

fn main() {
    input! {
      n: usize, d: usize,
      snake: [(usize, usize); n]
    }

    for k in 1..=d {
        let maximum = snake.iter().map(|&(t, l)| t * (l + k)).max().unwrap_or(0);
        println!("{maximum}")
    }
}
