use proconio::input;

fn main() {
    input! {
        _: usize, c1: char, c2: char,
        s: String,
    }

    let mut ans = vec![];
    for si in s.chars() {
        if si == c1 {
            ans.push(si);
        } else {
            ans.push(c2);
        }
    }

    println!("{}", ans.iter().collect::<String>());
}
