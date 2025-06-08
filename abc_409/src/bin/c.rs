use proconio::input;

fn main() {
    input! {
        n: usize, l: usize,
        d: [usize; n-1]
    }

    if l % 3 != 0 {
        println!("0");
        return;
    }

    let mut pos = vec![0; l];

    let mut buf = 0;
    pos[0] = 1;

    for di in d {
        buf = (buf + di) % l;
        pos[buf] += 1;
    }

    let dist = l / 3;
    let mut ans = 0usize;

    for a in 0..(l / 3) {
        let b = a + dist;
        let c = a + 2 * dist;

        ans += pos[a] * pos[b] * pos[c];
    }

    println!("{ans}");
}
