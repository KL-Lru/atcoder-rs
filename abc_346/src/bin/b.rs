use proconio::input;

const KEYBOARD: &str = "wbwbwwbwbwbw";

fn main() {
    let infinite_keyboard = KEYBOARD.repeat(100);

    input! {
        w: usize, b: usize,
    }

    let len = w + b;

    for win in infinite_keyboard.chars().collect::<Vec<_>>().windows(len) {
        if count(win.iter().collect::<String>()) == (w, b) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

fn count(s: String) -> (usize, usize) {
    (
        s.chars().filter(|&c| c == 'w').count(),
        s.chars().filter(|&c| c == 'b').count(),
    )
}
