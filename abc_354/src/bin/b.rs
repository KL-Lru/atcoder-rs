use proconio::input;

fn main() {
    input! {
        n: usize,
        mut players: [(String, usize); n],
    }

    players.sort();
    let sum = players.iter().map(|v| v.1).sum::<usize>();

    let winner = sum % n;

    println!("{}", players[winner].0);
}
