use proconio::input;

fn main() {
    input! {
       a: usize, b: usize,
    }

    let div = a as f64 / b as f64;
    let floor = div.floor();
    let ceil = div.ceil();

    if (div - floor).abs() > (div - ceil).abs() {
        println!("{ceil}");
    } else {
        println!("{floor}");
    }
}
