use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut v = vec![];

    for ai in a {
        v.push(ai);
        merge(&mut v);
    }

    println!("{}", v.len());
}

fn merge(v: &mut Vec<usize>) {
    while v.len() > 1 {
        let l = v.len();
        if v[l - 1] != v[l - 2] {
            break;
        } else {
            let val = v[l - 1] + 1;
            v.pop();
            v.pop();
            v.push(val);
        }
    }
}
