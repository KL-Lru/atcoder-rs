use core::panic;

use proconio::input;

fn main() {
    input! {
      n: usize, q: usize
    }

    let mut count = 0;
    let mut nests = (0..=n).collect::<Vec<_>>();
    let mut count_nests = vec![1; n + 1];

    for _ in 0..q {
        input! { t: usize }

        match t {
            1 => {
                input! { p: usize, h: usize  }
                let nest = nests[p];
                nests[p] = h;

                count_nests[nest] -= 1;
                count_nests[h] += 1;

                if count_nests[nest] == 1 {
                    count -= 1;
                }
                if count_nests[h] == 2 {
                    count += 1;
                }
            }
            2 => {
                println!("{count}");
            }
            _ => {
                panic!("unreachable")
            }
        }
    }
}
