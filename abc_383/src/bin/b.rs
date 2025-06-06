use proconio::{input, marker::Chars};

fn main() {
    input! {
      h: usize, w: usize, d: usize,
      field: [Chars; h]
    }

    let mut ans = 0;

    for x1 in 0..h {
        for y1 in 0..w {
            if field[x1][y1] == '#' {
                continue;
            }

            for x2 in 0..h {
                for y2 in 0..w {
                    if field[x2][y2] == '#' || (x1 == x2 && y1 == y2) {
                        continue;
                    }

                    ans = ans.max(calc(&field, (x1, y1), (x2, y2), d));
                }
            }
        }
    }

    println!("{ans}");
}

fn calc(field: &[Vec<char>], p1: (usize, usize), p2: (usize, usize), d: usize) -> usize {
    let mut count = 0;

    let h = field.len();
    let w = field[0].len();

    for x in 0..h {
        for y in 0..w {
            if field[x][y] == '#' {
                continue;
            }

            if distance(p1, (x, y)) <= d || distance(p2, (x, y)) <= d {
                count += 1;
            }
        }
    }

    count
}

fn distance(p1: (usize, usize), p2: (usize, usize)) -> usize {
    let dx = (p1.0 as isize - p2.0 as isize).unsigned_abs();
    let dy = (p1.1 as isize - p2.1 as isize).unsigned_abs();

    dx + dy
}
