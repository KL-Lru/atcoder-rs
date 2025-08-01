use proconio::input;

fn main() {
    input! {
        n: usize,
        d: [(isize, isize); n],
    }

    for i in 0..n {
        let mut v = (0, 0);

        for j in 0..n {
            if i == j {
                continue;
            }

            let dist = distance(d[i], d[j]);
            if v.1 < dist {
                v = (j + 1, dist);
            }
        }

        println!("{}", v.0);
    }
}

fn distance(x: (isize, isize), y: (isize, isize)) -> usize {
    ((x.0 - y.0).pow(2) + (x.1 - y.1).pow(2)) as usize
}
