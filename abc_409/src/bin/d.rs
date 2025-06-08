use proconio::{input, marker::Chars};

fn main() {
    input! { t: usize };

    for _ in 0..t {
        input! {
            n: usize,
            mut s: Chars,
        }

        let mut src = n.saturating_sub(2);
        for i in 0..(n - 1) {
            if s[i] > s[i + 1] {
                src = i;
                break;
            }
        }

        let mut dist = n;
        for j in src..n {
            if s[src] < s[j] {
                dist = j;
                break;
            }
        }

        let buf = s[src];
        s.insert(dist, buf);
        s.remove(src);

        println!("{}", s.iter().collect::<String>());
    }
}
