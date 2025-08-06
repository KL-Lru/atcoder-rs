use proconio::input;
use regex::Regex;

fn main() {
    input! {
        s: String
    }

    let reg = Regex::new(r"\|.*\|").expect("Fixed REGEXP");
    println!("{}", reg.replace(s.as_str(), ""));
}
