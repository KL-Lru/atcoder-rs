use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize, w: usize, x: usize, y: usize,
        s: [Chars; h],
        t: Chars,
    }

    let mut houses = HashSet::<(usize, usize)>::new();
    let mut current = (x - 1, y - 1);

    for ti in t {
        let (dx, dy) = match ti {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => continue,
        };

        let new_x = current.0 as isize + dx;
        let new_y = current.1 as isize + dy;

        if new_x < 0 || new_x >= h as isize || new_y < 0 || new_y >= w as isize {
            continue;
        }
        if s[new_x as usize][new_y as usize] == '#' {
            continue;
        }

        current = (new_x as usize, new_y as usize);

        if s[current.0][current.1] == '@' {
            houses.insert(current);
        }
    }

    println!("{} {} {}", current.0 + 1, current.1 + 1,houses.len());
}
