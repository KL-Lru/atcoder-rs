use proconio::input;

fn main() {
    input! {
        scores: [usize; 5]
    }

    let mut results = (1..=31)
        .map(|i| {
            let mut score = 0;
            let mut name = String::new();
            for j in 0..5u8 {
                if i & (1 << j) != 0 {
                    score += scores[j as usize];
                    name.push_str(&format!("{}", (b'A' + j) as char));
                }
            }

            (score, name)
        })
        .collect::<Vec<_>>();

    results.sort_by(|a, b| {
        if b.0 == a.0 {
            a.1.cmp(&b.1)
        } else {
            b.0.cmp(&a.0)
        }
    });

    for (_, name) in results {
        println!("{name}");
    }
}
