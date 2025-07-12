use proconio::input;

fn main() {
    input! {
       n: usize,
       rl: [(String, usize); n]
    }

    let mut ans = "".to_string();

    for (c, l) in rl {
        if ans.len() + l > 100 {
            println!("Too Long");
            return;
        }
        ans += &c.repeat(l);
    }

    println!("{ans}");
}
