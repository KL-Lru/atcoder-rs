use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; m],
        x: [[usize; m]; n],
    }

    let mogu = x.iter().fold(vec![0; m], |mut acc, row| {
        for (i, &val) in row.iter().enumerate() {
            acc[i] += val;
        }
        acc
    });

    for i in 0..m {
        if mogu[i] < a[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
