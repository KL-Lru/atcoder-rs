use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        r: [usize; n],
    }

    let mut v = vec![1; n];

    while v[0] <= r[0] {
        if v.iter().sum::<usize>() % k == 0 {
            println!("{}", v.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
        }
        next(&mut v, &r);
    }
}

fn next(v: &mut [usize], r: &[usize]) {
    v[v.len() - 1] += 1;

    let mut i = v.len() - 1;
    while i > 0 {
        if v[i] > r[i] {
            v[i] = 1;
            v[i - 1] += 1;
            i -= 1;
        } else {
            break;
        }
    }
}
