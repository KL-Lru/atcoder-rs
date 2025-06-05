use proconio::input;

fn main() {
    input! {
        n: usize, s: usize,
        a: [usize; n],
    }

    let sum = a.iter().sum::<usize>();
    let t = s % sum;

    let b = a.repeat(2).iter().fold(vec![], |mut acc, &x| {
        match acc.last() {
            Some(&last) => acc.push(x + last),
            None => acc.push(x),
        }

        acc
    });

    for i in 0..b.len() {
        if b.binary_search(&(b[i] + t)).is_ok() {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
