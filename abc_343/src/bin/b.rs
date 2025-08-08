use proconio::input;

fn main() {
    input! {
        n: usize,
        edge: [[usize; n]; n],
    }

    for i in 0..n {
        let mut v = vec![];
        for j in 0..n {
            if edge[i][j] == 1 {
                v.push((j + 1).to_string());
            }
        }
        println!("{}", v.join(" "));
    }
}
