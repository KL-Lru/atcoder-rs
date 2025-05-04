use proconio::{input, marker::Chars};

fn main() {
    input! {
      n: usize,
      mut s: [Chars; n],
      t: [Chars; n],
    }

    let mut ans = 1e+9 as usize;

    for rot in 0..4 {
        let count = reverse_count(&s, &t);
        ans = ans.min(count + rot);

        s = rotate_90(&s);
    }

    println!("{ans}");
}

fn reverse_count(s: &Vec<Vec<char>>, t: &Vec<Vec<char>>) -> usize {
    let n = s.len();
    let mut count = 0;

    for x in 0..n {
        for y in 0..n {
            if s[x][y] != t[x][y] {
                count += 1;
            }
        }
    }

    count
}

fn rotate_90(s: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let n = s.len();
    let mut rotated = vec![vec![' '; n]; n];

    for x in 0..n {
        for y in 0..n {
            rotated[y][(n - 1) - x] = s[x][y];
        }
    }

    rotated
}
