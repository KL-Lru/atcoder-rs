use proconio::input;
use proconio::marker::Chars;
const DX: [usize; 4] = [0, 1, 0, 1];
const DY: [usize; 4] = [0, 0, 1, 1];

fn is_vertex(field: &Vec<Vec<char>>, point: (usize, usize)) -> bool {
    let mut cnt: i8 = 0;
    for i in 0..4 {
        if field[point.0 + DX[i]][point.1 + DY[i]] == '#' {
            cnt += 1;
        }
    }
    match cnt {
        1 | 3 => return true,
        _ => return false,
    }
}

fn main() {
    input! {
      h: usize,
      w: usize,
      s: [Chars; h],
    }

    let mut ans = 0;
    for x in 0..(h - 1) {
        for y in 0..(w - 1) {
            if is_vertex(&s, (x, y)) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
