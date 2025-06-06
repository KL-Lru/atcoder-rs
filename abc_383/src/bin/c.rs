use std::collections::VecDeque;

use proconio::{input, marker::Chars};

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn main() {
    input! {
        h: usize, w: usize, d: usize,
        mut field: [Chars; h],
    }

    let mut queue = VecDeque::new();

    for (x, row) in field.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            if cell == 'H' {
                queue.push_back((x, y, 0));
            }
        }
    }

    while let Some((x, y, dist)) = queue.pop_front() {
        if dist > d || field[x][y] == '#' {
            continue;
        }

        field[x][y] = 'W';

        if dist == d {
            continue;
        }

        for (dx, dy) in DIRECTIONS {
            if outbound(x, y, dx, dy, h, w) {
                continue;
            }

            let cx = (x as isize + dx) as usize;
            let cy = (y as isize + dy) as usize;

            if field[cx][cy] == '.' || field[cx][cy] == 'H' {
                queue.push_back((cx, cy, dist + 1));
            }
        }
    }

    let ans = field.into_iter().flatten().filter(|&c| c == 'W').count();

    println!("{ans}");
}

fn outbound(x: usize, y: usize, dx: isize, dy: isize, h: usize, w: usize) -> bool {
    let cx = x as isize + dx;
    let cy = y as isize + dy;

    cx < 0 || cx >= h as isize || cy < 0 || cy >= w as isize
}
