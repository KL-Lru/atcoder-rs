use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize, w: usize,
        mut s: [Chars; h],
    }

    let mut q = VecDeque::new();

    for (x, row) in s.iter().enumerate() {
        for (y, &cell) in row.iter().enumerate() {
            if cell == 'E' {
                q.push_back((x, y, 'E'));
            }
        }
    }

    while let Some((x, y, c)) = q.pop_front() {
        if s[x][y] == '.' {
            s[x][y] = c;
        } else if s[x][y] != 'E' {
            continue;
        }

        if x > 0 && s[x - 1][y] == '.' {
            q.push_back((x - 1, y, 'v'));
        }
        if x < h - 1 && s[x + 1][y] == '.' {
            q.push_back((x + 1, y, '^'));
        }
        if y > 0 && s[x][y - 1] == '.' {
            q.push_back((x, y - 1, '>'));
        }
        if y < w - 1 && s[x][y + 1] == '.' {
            q.push_back((x, y + 1, '<'));
        }
    }

    println!(
        "{}",
        s.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    )
}
