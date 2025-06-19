use std::collections::HashSet;

use proconio::input;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

struct Field {
    h: usize,
    w: usize,
}

impl Field {
    fn new(h: usize, w: usize) -> Self {
        Field { h, w }
    }

    fn transfer(&self, point: &Point, dx: isize, dy: isize) -> Option<Point> {
        let new_x = point.x as isize + dx;
        let new_y = point.y as isize + dy;

        if new_x < 1 || new_x > self.h as isize || new_y < 1 || new_y > self.w as isize {
            None
        } else {
            Some(Point::new(new_x as usize, new_y as usize))
        }
    }
}

fn main() {
    input! {
        n: usize, m: usize,
        p: [(usize, usize); m],
    }

    let field = Field::new(n, n);
    let directions = [
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
    ];

    let mut reachable = HashSet::new();

    for (x, y) in p {
        let point = Point::new(x, y);
        reachable.insert(point.clone());
        for &(dx, dy) in &directions {
            if let Some(new_point) = field.transfer(&point, dx, dy) {
                reachable.insert(new_point);
            }
        }
    }

    println!("{}", n.pow(2) - reachable.len());
}
