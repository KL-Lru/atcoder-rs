use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n]
    }

    let mut ans = 1;

    for i in 0..n {
        ans = ans.max(max_step_count(&h, i));
    }

    println!("{ans}");
}

fn max_step_count(h: &[usize], start: usize) -> usize {
    let mut cnt = 1;

    for step in 1..h.len() {
        let current_count = same_height_count(h, start, step);
        cnt = cnt.max(current_count);
    }

    cnt
}

fn same_height_count(h: &[usize], start: usize, step: usize) -> usize {
    let mut cnt = 1;
    let mut k = start + step;
    while k < h.len() {
        if h[start] == h[k] {
            cnt += 1;
        } else {
            break;
        }
        k += step;
    }

    cnt
}
