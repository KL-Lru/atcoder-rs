use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut logged_in = false;
    let mut ans = 0;
    for si in s {
        match (si.as_str(), logged_in) {
            ("login", _) => logged_in = true,
            ("logout", _) => logged_in = false,
            ("private", false) => ans += 1,
            _ => {}
        }
    }

    println!("{ans}");
}
