use proconio::input;

fn main() {
    input! { mut a: [usize; 5] }

    let mut sorted = a.clone();
    sorted.sort();

    for i in 0..4 {
        a.swap(i, i + 1);

        if a == sorted {
            println!("Yes");
            return;
        } else {
            a.swap(i, i + 1);
        }
    }

    println!("No");
}
