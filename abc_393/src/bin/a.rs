use proconio::input;

fn main() {
    input! {
        s1: String,
        s2: String,
    }

    let ans = match (s1.as_str(), s2.as_str()) {
        ("sick", "sick") => 1,
        ("sick", "fine") => 2,
        ("fine", "sick") => 3,
        _ => 4,
    };

    println!("{ans}");
}
