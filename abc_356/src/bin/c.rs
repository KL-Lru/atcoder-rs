use proconio::input;

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        test: [([usize], char); m],
    }

    let mut pattern = pattern_vec(n);

    for (select, result) in test {
        let select = select.iter().map(|&x| x - 1).collect::<Vec<_>>();

        pattern.retain(|bits| match result {
            'o' => select.iter().filter(|&&i| bits[i]).count() >= k,
            'x' => select.iter().filter(|&&i| bits[i]).count() < k,
            _ => unreachable!(),
        });
    }

    println!("{}", pattern.len());
}

fn pattern_vec(n: usize) -> Vec<Vec<bool>> {
    let mut pattern = vec![];

    for i in 0..(1 << n) {
        let mut bits = vec![];
        for j in 0..n {
            bits.push((i >> j) & 1 == 1);
        }
        pattern.push(bits);
    }

    pattern
}
