use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let ans = match s.as_str() {
        "red" => "SSS",
        "blue" => "FFF",
        "green" => "MMM",
        _ => "Unknown",
    };

    println!("{ans}");
}
