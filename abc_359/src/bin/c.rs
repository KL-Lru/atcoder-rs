use proconio::input;

fn main() {
    input! {
      sx: u64, sy: u64,
      tx: u64, ty: u64,
    }

    let diffx = sx.abs_diff(tx);
    let diffy = sy.abs_diff(ty);

    let mut ans = diffy;
    if diffx > diffy {
        ans += diffx.abs_diff(diffy) / 2;

        if diffx.abs_diff(diffy) % 2 == 1 {
            if sx < tx && (sx + sy) % 2 == 1 {
                ans += 1;
            }
            if sx > tx && (sx + sy) % 2 == 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
