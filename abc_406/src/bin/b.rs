use proconio::input;

fn main() {
    input! {
      n: usize, k: usize,
      a: [u64; n],
    }

    let mut current = 1;

    for ai in a {
        if (ai.to_string().len() + current.to_string().len() - 1) > k {
            current = 1;
        } else {
            current *= ai;
        }

        if current.to_string().len() > k {
            current = 1;
        }
    }

    println!("{current}");
}
