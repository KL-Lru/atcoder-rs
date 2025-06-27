use proconio::input;

fn main() {
    input! {
        m: usize,
    }

    let md = m % 3;
    let mut ans = vec![0; md];

    let mut rem = m - md;
    while rem > 0 {
        let mut pw = 1;
        while 3usize.pow(pw + 1) <= rem {
            pw += 1;
        }
        ans.push(pw);
        rem -= 3usize.pow(pw);
    }

    println!("{}", ans.len());
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
