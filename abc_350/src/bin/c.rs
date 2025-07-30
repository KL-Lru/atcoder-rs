use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut pos = vec![0; n];
    for (i, &x) in a.iter().enumerate() {
        pos[x - 1] = i;
    }

    let mut ans = vec![];

    for i in 0..n {
        if pos[i] == i {
            continue;
        }

        let swap_pos = a[i] - 1;
        ans.push((pos[swap_pos] + 1, pos[i] + 1));
        a.swap(pos[i], pos[swap_pos]);
        pos.swap(i, swap_pos);
    }

    println!("{}", ans.len());
    for (x, y) in ans {
        println!("{} {}", x, y);
    }
}
