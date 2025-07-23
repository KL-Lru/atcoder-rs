use proconio::input;

fn main() {
    input! {
        n: usize, k: usize,
        a: [usize; n],
    }

    let groups = a.iter().fold(vec![], |mut acc, &ai| {
        match acc.last_mut() {
            Some(v) if *v + ai <= k => *v += ai,
            _ => acc.push(ai),
        }
        acc
    });

    println!("{}", groups.len());
}
