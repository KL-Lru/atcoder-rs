use proconio::input;

fn main() {
    input! {
        n: usize, k: usize, x: usize,
        s: [String; n],
    }

    let mut pattern = vec![];
    append("".to_string(), &s, &mut pattern, k);

    pattern.sort();

    println!("{}", pattern[x - 1]);
}

fn append(current: String, s: &[String], pattern: &mut Vec<String>, depth: usize) {
    if depth == 0 {
        pattern.push(current);
        return;
    }

    for i in 0..s.len() {
        let new_str = format!("{}{}", current, s[i]);
        append(new_str, s, pattern, depth - 1);
    }
}
