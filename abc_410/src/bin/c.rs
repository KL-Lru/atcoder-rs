use proconio::input;

fn main() {
    input! {
        n: usize, q: usize,
    }

    let mut arr = (1..=n).collect::<Vec<_>>();
    let mut rot = 0;

    for _ in 0..q {
        input! {
            t: usize
        }

        match t {
            1 => {
                input! {
                    p: usize, x: usize,
                }
                arr[(p - 1 + rot + n) % n] = x;
            }
            2 => {
                input! {
                    p: usize
                }
                println!("{}", arr[(p - 1 + rot + n) % n]);
            }
            3 => {
                input! {
                    k: usize
                }
                rot = (rot + k) % n;
            }
            _ => unreachable!()
        }
    }
}
