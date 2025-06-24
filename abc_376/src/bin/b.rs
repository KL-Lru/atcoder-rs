use proconio::input;

fn main() {
    input! {
        n: isize, q:usize,
        ops: [(char, isize); q],
    }

    let mut left = 1;
    let mut right = 2;
    let mut ans = 0;

    for (op, x) in ops {
        match op {
            'L' => {
                match (
                    clock_rotate(left, x, right, n),
                    rclock_rotate(left, x, right, n),
                ) {
                    (Some(rot), None) | (None, Some(rot)) => {
                        ans += rot;
                        left = x;
                    }
                    (Some(0), Some(0)) => { /* DO NOTHING */ }
                    _ => unreachable!(),
                }
            }
            'R' => {
                match (
                    clock_rotate(right, x, left, n),
                    rclock_rotate(right, x, left, n),
                ) {
                    (Some(rot), None) | (None, Some(rot)) => {
                        ans += rot;
                        right = x;
                    }
                    (Some(0), Some(0)) => { /* DO NOTHING */ }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{ans}");
}

fn clock_rotate(start: isize, mut target: isize, mut pin: isize, n: isize) -> Option<usize> {
    if target < start {
        target += n;
    }
    if pin < start {
        pin += n;
    }
    if start < pin && pin < target {
        return None;
    }

    Some(target.abs_diff(start))
}

fn rclock_rotate(start: isize, mut target: isize, mut pin: isize, n: isize) -> Option<usize> {
    if target > start {
        target -= n;
    }
    if pin > start {
        pin -= n;
    }
    if target < pin && pin < start {
        return None;
    }

    Some(target.abs_diff(start))
}
