use proconio::input;

fn main() {
    input! {
       x: f32
    }

    if x >= 38.0 {
        println!("1")
    } else if x >= 37.5 {
        println!("2")
    } else {
        println!("3")
    }
}
