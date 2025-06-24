use proconio::input;

const INF: usize = (1e+9 as usize) + 1;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n - 1],
    }

    a.sort();
    b.sort();

    let mut l = 0;
    let mut r = INF;
    while r - l > 1 {
        let mid = (l + r) / 2;

        if satisfy(mid, &a, &b) {
            r = mid;
        } else {
            l = mid;
        }
    }

    if r != INF {
        println!("{r}");
    } else {
        println!("-1");
    }
}

fn satisfy(insertion: usize, a: &Vec<usize>, b: &Vec<usize>) -> bool {
    let idx = b.partition_point(|&x| x <= insertion);
    let mut tb = b.clone();
    tb.insert(idx, insertion);

    for i in 0..a.len() {
        if a[i] > tb[i] {
            return false;
        }
    }

    true
}
