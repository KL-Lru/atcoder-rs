use std::collections::HashSet;

use proconio::input;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn next(&mut self, c: char) {
        match c {
            // Reverse the direction
            'N' => self.x += 1,
            'W' => self.y += 1,
            'S' => self.x -= 1,
            'E' => self.y -= 1,
            _ => panic!("Invalid Input"),
        }
    }
}

fn main() {
    input! {
        _: i32, r: i32, c: i32,
        s: String,
    }

    let mut human = Point::new(r, c);
    let mut fire = Point::new(0, 0);

    let mut smoke_point = HashSet::new();
    smoke_point.insert(fire);

    for c in s.chars() {
        human.next(c);
        fire.next(c);

        smoke_point.insert(fire);
        if smoke_point.contains(&human) {
            print!("1");
        } else {
            print!("0");
        }
    }
    println!();
}
