use proconio::input;

fn main() {
    input! {
      s: String,
    }

    let alphabets = b'a'..=b'z';

    for alphabet in alphabets {
        if s.contains(alphabet as char) {
            continue;
        }

        println!("{}", alphabet as char);
        return;
    }
}
