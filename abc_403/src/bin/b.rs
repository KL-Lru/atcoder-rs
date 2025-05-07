use proconio::input;

fn main() {
    input! {
        t: String,
        u: String,
    }

    for ti in 0..(t.len() - u.len() + 1) {
        let mut satisfy = true;
        for ui in 0..u.len() {
            if t.chars().nth(ti + ui) != u.chars().nth(ui) && t.chars().nth(ti + ui) != Some('?') {
                satisfy = false;
                break;
            }
        }
        if satisfy {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
