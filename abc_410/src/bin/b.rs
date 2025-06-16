use proconio::input;

fn main() {
    input! {
        n: usize, q: usize,
        x: [usize; q],
    }

    let mut boxes = vec![0; n];

    for xi in x {
        if xi == 0 {
            let min = *boxes.iter().min().expect("cannot empty");
            let min_index = boxes
                .iter()
                .position(|&i| i == min)
                .expect("cannot lost min index");
            print!("{} ", min_index + 1);
            boxes[min_index] += 1;
        } else {
            boxes[xi - 1] += 1;
            print!("{xi} ");
        }
    }
    println!();
}
