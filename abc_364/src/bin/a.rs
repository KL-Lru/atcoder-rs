use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut so_sweet = false;
    for i in 0..n {
        match s[i].as_str() {
            "sweet" if so_sweet && i != n - 1 => {
                println!("No");
                return;
            }
            "sweet" => so_sweet = true,
            "salty" => so_sweet = false,
            _ => unreachable!(),
        }
    }

    println!("Yes");
}
