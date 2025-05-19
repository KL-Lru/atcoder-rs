use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut increasing = Vec::<u64>::new();

    let mut current = 0;
    let mut prev = p[0];
    for &pi in p.iter().take(n).skip(1) {
        if pi > prev {
            current += 1;
        } else {
            if current > 0 {
                increasing.push(current);
            }
            current = 0;
        }

        prev = pi;
    }
    if current > 0 {
        increasing.push(current);
    }

    if increasing.len() < 2 {
        println!("0");
        return;
    }

    let mut ans = 0;
    for i in 0..(increasing.len() - 1) {
        ans += increasing[i] * increasing[i + 1];
    }

    println!("{ans}");
}
