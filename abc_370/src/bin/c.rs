use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        t: Chars,
    }

    let mut diff = Vec::new();
    for (i, (si, ti)) in s.clone().into_iter().zip(t.clone().into_iter()).enumerate() {
        if si != ti {
            diff.push((i, si, ti));
        }
    }

    let mut change = Vec::new();
    while s != t {
        let mut changed = false;
        for &(idx, si, ti) in &diff {
            if changed {
                break;
            }
            if s[idx] != ti && ti < si {
                s[idx] = ti;
                change.push(s.clone());
                changed = true;
            }
        }

        for &(idx, _, ti) in diff.iter().rev() {
            if changed {
                break;
            }
            if s[idx] != ti {
                s[idx] = ti;
                change.push(s.clone());
                changed = true;
            }
        }
    }

    println!("{}", change.len());
    for c in change {
        println!("{}", c.iter().collect::<String>());
    }
}
