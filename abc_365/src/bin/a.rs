use proconio::input;

fn main() {
    input! {
        y: usize,
    }

    let ans = match y {
        y if y % 400 == 0 => 366,
        y if y % 100 == 0 => 365,
        y if y % 4 == 0 => 366,
        _ => 365,
    };

    println!("{ans}");
}
