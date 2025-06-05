use proconio::input;

fn main() {
    input! {
        n: usize, mut r: isize,
        results: [(usize, isize); n],
    }

    for (d, a) in results {
        match d {
            1 if (1600..2800).contains(&r) => {
                r += a;
            }
            2 if (1200..2400).contains(&r) => {
                r += a;
            }
            _ => {}
        }
    }

    println!("{r}")
}
