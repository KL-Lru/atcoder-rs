use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n]
    }

    let diff = a.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();

    let mut ans = n;
    let run = run_length(&diff);

    for (_, count) in run {
        ans += (count * (count + 1)) / 2;
    }

    println!("{ans}");
}

fn run_length(s: &[isize]) -> Vec<(isize, usize)> {
    s.iter().fold(Vec::new(), |mut acc, si| {
        if let Some((last, count)) = acc.last_mut() {
            if last == si {
                *count += 1;
            } else {
                acc.push((*si, 1));
            }
        } else {
            acc.push((*si, 1));
        }
        acc
    })
}
