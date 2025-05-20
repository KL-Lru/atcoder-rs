use proconio::input;

fn main() {
    input! {
        s: String
    }

    let mut ans = String::new();
    let mut s_buf = String::new();
    let mut ans_buf = String::new();
    for si in s.chars() {
        if si == 'W' {
            s_buf.push(si);
            if ans_buf.is_empty() {
                ans_buf.push('A');
            } else {
                ans_buf.push('C');
            }
        } else if si == 'A' && !ans_buf.is_empty() {
            ans_buf.push('C');
            ans.push_str(&ans_buf);
            ans_buf.clear();
            s_buf.clear();
        } else {
            ans.push_str(&s_buf);
            ans.push(si);
            ans_buf.clear();
            s_buf.clear();
        }
    }
    if !s_buf.is_empty() {
        ans.push_str(&s_buf);
    }

    println!("{ans}");
}
