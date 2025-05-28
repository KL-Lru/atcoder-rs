use proconio::{input, marker::Chars};

struct PaintArea {
    top: usize,
    bottom: usize,
    left: usize,
    right: usize,
}

impl PaintArea {
    fn adjust(&mut self, x: usize, y: usize) {
        self.top = self.top.min(x);
        self.bottom = self.bottom.max(x);
        self.left = self.left.min(y);
        self.right = self.right.max(y);
    }
}

fn main() {
    input! {
        h: usize, w: usize,
        s: [Chars; h],
    }

    let mut area = PaintArea {
        top: h - 1,
        bottom: 0,
        left: w - 1,
        right: 0,
    };

    for x in 0..h {
        for y in 0..w {
            if s[x][y] == '#' {
                area.adjust(x, y);
            }
        }
    }

    for x in area.top..=area.bottom {
        for y in area.left..=area.right {
            if s[x][y] == '.' {
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
