use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut stack = vec![0; 100];

    for _ in 0..q {
        input! { t: usize }

        match t {
            1 => {
                input! {x: usize}
                stack.push(x);
            }
            2 => {
                println!("{}", stack.pop().expect("Invalid Input"));
            }
            _ => panic!("Invalid Input"),
        }
    }
}
