use proconio::input;

fn main() {
    input! {
        _: usize,
        s: String,
        t: String,
    }

    let mut ans = 0;
    s.as_bytes().iter().zip(t.as_bytes()).for_each(|(s, t)| {
        if s != t {
            ans += 1;
        }
    });
    println!("{ans}");
}
