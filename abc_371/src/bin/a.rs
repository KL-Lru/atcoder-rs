use proconio::input;

fn main() {
    input! {
        sab: char,
        sac: char,
        sbc: char,
    }

    let mid = match (sab, sac, sbc) {
        ('<', '<', '<') | ('>', '>', '>') => "B",
        ('<', '<', '>') | ('>', '>', '<') => "C",
        ('<', '>', _) | ('>', '<', _) => "A",
        _ => unreachable!(),
    };
    println!("{mid}");
}
