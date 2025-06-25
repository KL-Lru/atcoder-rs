use proconio::{input, marker::Chars};

type MapFn = fn(usize, usize, usize) -> (usize, usize);

fn main() {
    input! {
        n: usize,
        mut field: [Chars; n],
    }

    let mut mapping = vec![vec![id as MapFn; n]; n];

    for i in 0..(n / 2) {
        let rot_fn = match (i + 1) % 4 {
            0 => id,
            1 => point_rotate,
            2 => |x, y, n| {
                let (x, y) = point_rotate(x, y, n);
                point_rotate(x, y, n)
            },
            3 => |x, y, n| {
                let (x, y) = point_rotate(x, y, n);
                let (x, y) = point_rotate(x, y, n);
                point_rotate(x, y, n)
            },
            _ => unreachable!(),
        };

        let x = [i, n - i - 1];
        let y = [i, n - i - 1];
        for dx in x {
            for dy in i..(n - i) {
                mapping[dx][dy] = rot_fn;
            }
        }
        for dy in y {
            for dx in i..(n - i) {
                mapping[dx][dy] = rot_fn;
            }
        }
    }

    let mut ans = vec![vec!['?'; n]; n];
    for x in 0..n {
        for y in 0..n {
            let (nx, ny) = mapping[x][y](x, y, n);
            ans[nx][ny] = field[x][y];
        }
    }

    for row in ans {
        println!("{}", row.iter().collect::<String>());
    }
}

fn id(x: usize, y: usize, _: usize) -> (usize, usize) {
    (x, y)
}

fn point_rotate(x: usize, y: usize, n: usize) -> (usize, usize) {
    (y, n - 1 - x)
}
