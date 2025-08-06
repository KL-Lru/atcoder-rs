use proconio::input;

fn main() {
    let mut a = vec![];

    while a.last() != Some(&0) {
        input! { ai: usize }
        a.push(ai);
    }

    a.reverse();

    for ai in a {
        println!("{ai}");
    }
}
