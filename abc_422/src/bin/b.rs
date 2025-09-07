use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize, w: usize,
        field: [Chars; h],
    }

    for x in 0..h {
        for y in 0..w {
            if field[x][y] == '.' {
                continue;
            }

            let bc = [
                0 < x && field[x - 1][y] == '#',
                x < h - 1 && field[x + 1][y] == '#',
                0 < y && field[x][y - 1] == '#',
                y < w - 1 && field[x][y + 1] == '#',
            ]
            .iter()
            .filter(|&&f| f)
            .count();

            if bc == 2 || bc == 4 {
                continue;
            }

            println!("No");
            return;
        }
    }

    println!("Yes");
}
