use proconio::{input, marker::Chars};

fn main() {
    input! { t: usize }

    for _ in 0..t {
        solve();
    }
}
const INF: usize = 1e+9 as usize;

fn solve() {
    input! {
        h: usize, w: usize,
        field: [Chars; h],
    }

    let bit_field = to_bits(field);

    let pat_max = 2usize.pow(w as u32);
    let mut dp = vec![vec![INF; pat_max]; h];

    for pat in 0..pat_max {
        if can_make(bit_field[0], pat) {
            dp[0][pat] = changed_bit(bit_field[0], pat);
        }
    }

    for row in 1..h {
        for pat in 0..pat_max {
            if can_make(bit_field[row], pat) {
                dp[row][pat] = (0..pat_max)
                    .filter(|&prev_pat| dp[row - 1][prev_pat] != INF)
                    .filter(|&prev_pat| !has_black_region(prev_pat, pat, w))
                    .map(|prev_pat| dp[row - 1][prev_pat] + changed_bit(bit_field[row], pat))
                    .min()
                    .unwrap_or(INF);
            }
        }
    }

    println!("{}", dp[h - 1].iter().min().expect("Not empty"));
}

fn can_make(source: usize, dist: usize) -> bool {
    !source & dist == 0
}

fn changed_bit(source: usize, dist: usize) -> usize {
    (source & !dist).count_ones() as usize
}

fn has_black_region(prev_pat: usize, current_pat: usize, w: usize) -> bool {
    let mut mask = 1 + (1 << 1);

    while mask < (1 << w) {
        if prev_pat & mask == mask && current_pat & mask == mask {
            return true;
        }
        mask <<= 1;
    }

    false
}

fn to_bits(field: Vec<Vec<char>>) -> Vec<usize> {
    field.into_iter().map(to_bit).collect()
}

fn to_bit(row: Vec<char>) -> usize {
    row.into_iter().enumerate().fold(
        0usize,
        |acc, (idx, c)| if c == '#' { acc + (1 << idx) } else { acc },
    )
}
