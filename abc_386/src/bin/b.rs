use proconio::input;

fn main() {
    input! {
        s: String,
    }

    println!("{}", s.replace("00", "0").len());
}
