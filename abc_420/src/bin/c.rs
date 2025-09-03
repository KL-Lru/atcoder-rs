use proconio::input;

fn main() {
    input! {
        n: usize, q: usize,
        mut a: [isize; n],
        mut b: [isize; n],
        queries: [(char, usize, isize); q],
    }

    let mut current_sm: isize = a.iter().zip(b.iter()).map(|(ai, bi)| ai.min(bi)).sum();

    for (c, x, v) in queries {
        let x = x - 1;

        match c {
            'A' => {
                current_sm += v.min(b[x]) - a[x].min(b[x]);

                a[x] = v;
            }
            'B' => {
                current_sm += v.min(a[x]) - a[x].min(b[x]);

                b[x] = v;
            }
            _ => unreachable!(),
        }
        println!("{current_sm}");
    }
}
