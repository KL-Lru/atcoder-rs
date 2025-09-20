use proconio::input;

fn main() {
    input! {
        n: usize, m: usize, k: usize,
        events: [(usize, usize); k],
    }

    let mut solves = vec![vec![false; m]; n];
    let mut ans = vec![];

    for (solver, problem) in events {
        let s = &mut solves[solver - 1];
        s[problem - 1] = true;

        if s.iter().all(|t| *t) {
            ans.push(solver.to_string());
        }
    }

    println!("{}", ans.join(" "));
}
