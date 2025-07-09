use proconio::{input, marker::Chars};

struct Point {
    x: usize,
    y: usize,
}

fn main() {
    input! {
        h: usize, w: usize,
        start: (usize, usize),
        fields: [Chars; h],
        x: Chars,
    }

    let mut pos = Point {
        x: start.0 - 1,
        y: start.1 - 1,
    };

    for xi in x {
        match xi {
            'L' => {
                if pos.y > 0 && fields[pos.x][pos.y - 1] != '#' {
                    pos.y -= 1;
                }
            }
            'R' => {
                if pos.y < w - 1 && fields[pos.x][pos.y + 1] != '#' {
                    pos.y += 1;
                }
            }
            'U' => {
                if pos.x > 0 && fields[pos.x - 1][pos.y] != '#' {
                    pos.x -= 1;
                }
            }
            'D' => {
                if pos.x < h - 1 && fields[pos.x + 1][pos.y] != '#' {
                    pos.x += 1;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{} {}", pos.x + 1, pos.y + 1);
}
