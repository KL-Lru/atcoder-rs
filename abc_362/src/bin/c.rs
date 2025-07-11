use proconio::input;

fn main() {
    input! {
        n: usize,
        c: [(isize, isize); n]
    }

    let (min, max) = c
        .iter()
        .fold((0, 0), |(min, max), &(a, b)| (min + a, max + b));

    if 0 < min || max < 0 {
        println!("No");
        return;
    }

    println!("Yes");
    let mut buf = min.abs();
    let mut ans = vec![];

    for (l, r) in c {
        let x = (l + buf).min(r);
        ans.push(x.to_string());
        buf -= x - l;
    }

    println!("{}", ans.join(" "));
}
