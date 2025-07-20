use proconio::input;

fn main() {
    input! {
        n: usize, m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut c = [
        a.into_iter()
            .map(|x| (x, 'a'))
            .collect::<Vec<(usize, char)>>(),
        b.into_iter()
            .map(|x| (x, 'b'))
            .collect::<Vec<(usize, char)>>(),
    ]
    .concat();

    c.sort();

    for ct in c.windows(2) {
        let x = ct[0];
        let y = ct[1];
        if x.1 == 'a' && y.1 == 'a' {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
