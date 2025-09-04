use proconio::input;

fn main() {
    input! {
        mut x: usize, mut y: usize,
    }

    for _ in 3..=10 {
        let buf = y;
        y = rev(x + y);
        x = buf;
    }

    println!("{y}");
}

fn rev(x: usize) -> usize {
    let s = x.to_string();
    let rs = s.chars().rev().collect::<String>();

    rs.parse::<usize>().expect("Cannot not number")
}
