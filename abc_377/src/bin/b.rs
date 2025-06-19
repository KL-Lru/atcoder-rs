use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        field: [Chars; 8]
    }

    let mut xs = HashSet::new();
    let mut ys = HashSet::new();

    for (x, row) in field.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            if cell == '#' {
                xs.insert(x);
                ys.insert(y);
            }
        }
    }

    println!("{}", (8 - xs.len()) * (8 - ys.len()));
}
