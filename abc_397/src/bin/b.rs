use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut sw = 0;
    let mut ans = 0;
    for (idx, si) in s.chars().enumerate() {
        match (si, (1 + idx + sw) % 2) {
            ('i', 0) => {
                ans += 1;
                sw = (sw + 1) % 2;
            }
            ('o', 1) => {
                ans += 1;
                sw = (sw + 1) % 2;
            }
            _ => {}
        }
    }
    if (ans + s.chars().count()) % 2 == 1 {
        ans += 1;
    }

    println!("{ans}");
}
