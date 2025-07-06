use proconio::input;

fn main() {
    input! { n: usize }

    let mut a = vec![vec![vec![0; n]; n]; n];

    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                input! { v: usize }
                a[x][y][z] = v;
            }
        }
    }

    for x in 0..n {
        for y in 0..n {
            for z in 0..n {
                a[x][y][z] = cumulative_sum(&a, x, y, z);
            }
        }
    }

    input! { q: usize }

    for _ in 0..q {
        input! { xi: usize, xo: usize, yi: usize, yo: usize, zi: usize, zo: usize }

        println!(
            "{}",
            calc_sum(&a, (xi - 1, xo - 1), (yi - 1, yo - 1), (zi - 1, zo - 1),)
        );
    }
}

fn cumulative_sum(a: &Vec<Vec<Vec<usize>>>, x: usize, y: usize, z: usize) -> usize {
    let mut result = a[x][y][z];

    if x > 0 {
        result += a[x - 1][y][z]
    }
    if y > 0 {
        result += a[x][y - 1][z]
    }
    if z > 0 {
        result += a[x][y][z - 1]
    }
    if x > 0 && y > 0 && z > 0 {
        result += a[x - 1][y - 1][z - 1]
    }
    if x > 0 && y > 0 {
        result -= a[x - 1][y - 1][z]
    }
    if x > 0 && z > 0 {
        result -= a[x - 1][y][z - 1]
    }
    if y > 0 && z > 0 {
        result -= a[x][y - 1][z - 1]
    }

    result
}

fn calc_sum(
    a: &Vec<Vec<Vec<usize>>>,
    xp: (usize, usize),
    yp: (usize, usize),
    zp: (usize, usize),
) -> usize {
    let (xi, yi, zi) = (xp.0, yp.0, zp.0);
    let (xo, yo, zo) = (xp.1, yp.1, zp.1);

    let mut result = a[xo][yo][zo];

    if xi > 0 && yi > 0 {
        result += a[xi - 1][yi - 1][zo];
    }
    if xi > 0 && zi > 0 {
        result += a[xi - 1][yo][zi - 1];
    }
    if yi > 0 && zi > 0 {
        result += a[xo][yi - 1][zi - 1];
    }
    if xi > 0 {
        result -= a[xi - 1][yo][zo];
    }
    if yi > 0 {
        result -= a[xo][yi - 1][zo];
    }
    if zi > 0 {
        result -= a[xo][yo][zi - 1];
    }
    if xi > 0 && yi > 0 && zi > 0 {
        result -= a[xi - 1][yi - 1][zi - 1];
    }

    result
}
