use proconio::input;

fn main() {
    input! {
        n: u64,
    }

    println!("{}", possible_b(n, 1) + possible_b(n, 2));
}

fn possible_b(n: u64, a: u32) -> u64 {
    bin_search(0, n, |b| calc_x(a, b) <= n)
}

fn bin_search(l: u64, r: u64, predicate: impl Fn(u64) -> bool) -> u64 {
    if r - l <= 1 {
        return l;
    }
    let mid = (l + r) / 2;
    if predicate(mid) {
        bin_search(mid, r, predicate)
    } else {
        bin_search(l, mid, predicate)
    }
}

fn calc_x(a: u32, b: u64) -> u64 {
    2u64.saturating_pow(a).saturating_mul(b.saturating_pow(2))
}
