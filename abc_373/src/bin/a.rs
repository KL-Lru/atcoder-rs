use proconio::input;

fn main() {
    input! {
      s: [String; 12],
    }

    let mut ans = 0;
    for i in 1..=12 {
        if s[i - 1].len() == i {
            ans += 1;
        }
    }

    println!("{ans}");
}
