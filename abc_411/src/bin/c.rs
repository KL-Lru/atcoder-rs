use proconio::input;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Black,
    White,
}

fn main() {
    input! {
        n: usize, q: usize,
        a: [usize; q]
    }

    let mut field = vec![Cell::White; n + 2];
    let mut ans = 0;

    for ai in a {
        match (field[ai - 1], field[ai], field[ai + 1]) {
            (Cell::Black, Cell::Black, Cell::Black) => ans += 1,
            (Cell::White, Cell::White, Cell::White) => ans += 1,
            (Cell::Black, Cell::White, Cell::Black) => ans -= 1,
            (Cell::White, Cell::Black, Cell::White) => ans -= 1,
            _ => {}
        }
        field[ai] = match field[ai] {
            Cell::Black => Cell::White,
            Cell::White => Cell::Black,
        };

        println!("{ans}");
    }
}
