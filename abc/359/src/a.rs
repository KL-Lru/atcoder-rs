use proconio::input;

fn main() {
    input! {
      n: u32,
      s: [String; n],
    }

    let ans = s.iter().filter(|si| si.as_str() == "Takahashi").count();
    println!("{}", ans);
}
