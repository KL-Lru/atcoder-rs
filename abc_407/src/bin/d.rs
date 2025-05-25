use std::ops::BitXor;

use proconio::input;

#[derive(Copy, Clone)]
struct Point {
    h: usize,
    w: usize,
    x: usize,
    y: usize,
}

impl Point {
    fn next(&self) -> Self {
        if self.has_next_x() {
            Self {
                x: self.x + 1,
                y: self.y,
                ..*self
            }
        } else if self.has_next_y() {
            Self {
                x: 0,
                y: self.y + 1,
                ..*self
            }
        } else {
            *self
        }
    }

    fn is_terminal(&self) -> bool {
        self.x == self.h - 1 && self.y == self.w - 1
    }

    fn has_next_x(&self) -> bool {
        self.x < self.h - 1
    }

    fn has_next_y(&self) -> bool {
        self.y < self.w - 1
    }
}

struct Field {
    has_domino: Vec<Vec<bool>>,
    score: Vec<Vec<usize>>,
}

impl Field {
    fn new(score: Vec<Vec<usize>>) -> Self {
        let (h, w) = Self::dimension(&score);
        let has_domino = vec![vec![false; w]; h];

        Self { score, has_domino }
    }

    fn put_x_domino(&mut self, p: &Point) {
        self.has_domino[p.x][p.y] = true;
        self.has_domino[p.x + 1][p.y] = true;
    }

    fn put_y_domino(&mut self, p: &Point) {
        self.has_domino[p.x][p.y] = true;
        self.has_domino[p.x][p.y + 1] = true;
    }

    fn remove_x_domino(&mut self, p: &Point) {
        self.has_domino[p.x][p.y] = false;
        self.has_domino[p.x + 1][p.y] = false;
    }

    fn remove_y_domino(&mut self, p: &Point) {
        self.has_domino[p.x][p.y] = false;
        self.has_domino[p.x][p.y + 1] = false;
    }

    fn calc(&self) -> usize {
        let (h, w) = Self::dimension(&self.has_domino);

        let mut score = 0;
        for x in 0..h {
            for y in 0..w {
                if !self.has_domino[x][y] {
                    score = score.bitxor(self.score[x][y]);
                }
            }
        }

        score
    }

    fn dimension<T>(field: &[Vec<T>]) -> (usize, usize) {
        (field.len(), field[0].len())
    }
}

fn main() {
    input! {
        h: usize, w: usize,
        a: [[usize; w]; h],
    }

    let mut field = Field::new(a);
    let start = Point { h, w, x: 0, y: 0 };

    let ans = dfs(start, &mut field);

    println!("{ans}");
}

fn dfs(p: Point, field: &mut Field) -> usize {
    if p.is_terminal() {
        return field.calc();
    }

    let mut score = 0;
    let has_domino = field.has_domino[p.x][p.y];
    // 縦に置く
    if !has_domino && p.has_next_x() && !field.has_domino[p.x + 1][p.y] {
        field.put_x_domino(&p);
        score = score.max(dfs(p.next(), field));
        field.remove_x_domino(&p);
    }

    // 横に置く
    if !has_domino && p.has_next_y() && !field.has_domino[p.x][p.y + 1] {
        field.put_y_domino(&p);
        score = score.max(dfs(p.next(), field));
        field.remove_y_domino(&p);
    }

    // 置かない
    score = score.max(dfs(p.next(), field));

    score
}
