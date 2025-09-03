use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize, m: usize,
        s: [Chars; n],
    }

    let mut winners = vec![];

    for mi in 0..m {
        let votes = s.iter().map(|v| v[mi]).collect::<Vec<char>>();
        let zeros = votes.iter().filter(|v| **v == '0').count();
        let ones = votes.iter().filter(|v| **v == '1').count();

        let win = match (zeros, ones) {
            (0, _) => '1',
            (_, 0) => '0',
            (x, y) => {
                if x < y {
                    '0'
                } else {
                    '1'
                }
            }
        };

        winners.push(win);
    }

    let mut scores = vec![0; n];

    for ni in 0..n {
        for mi in 0..m {
            if s[ni][mi] == winners[mi] {
                scores[ni] += 1;
            }
        }
    }

    let max_score = scores.iter().max().expect("Not empty");
    let ans = scores
        .iter()
        .enumerate()
        .filter_map(|(idx, s)| {
            if s == max_score {
                Some((idx + 1).to_string())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    println!("{}", ans.join(" "))
}
