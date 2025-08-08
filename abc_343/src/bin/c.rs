use proconio::input;

fn main() {
    input! { n: usize }

    let mut ans = 1usize;
    let mut buf = 1usize;
    while buf.pow(3) <= n {
        let current = buf.pow(3);
        if is_palindrome(current) {
            ans = current;
        }
        buf += 1;
    }

    println!("{ans}");
}

fn is_palindrome(mut x: usize) -> bool {
    let mut v = vec![];

    while x != 0 {
        v.push(x % 10);
        x /= 10;
    }

    v == v.clone().into_iter().rev().collect::<Vec<_>>()
}
