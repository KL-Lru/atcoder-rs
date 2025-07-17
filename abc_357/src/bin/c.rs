use proconio::input;

fn main() {
    input! { n: usize }

    let result = render_carpet(n);
    println!(
        "{}",
        result
            .into_iter()
            .map(|row| row
                .into_iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(""))
            .collect::<Vec<_>>()
            .join("\n")
    )
}

fn render_carpet(n: usize) -> Vec<Vec<char>> {
    if n == 0 {
        return vec![vec!['#']];
    }

    let block_size = 3usize.pow(n as u32 - 1);
    let around = render_carpet(n - 1);
    let center = vec![vec!['.'; block_size]; block_size];

    combine_vertical([
        &combine_horizontal([&around, &around, &around]),
        &combine_horizontal([&around, &center, &around]),
        &combine_horizontal([&around, &around, &around]),
    ])
}

fn combine_horizontal(mat: [&Vec<Vec<char>>; 3]) -> Vec<Vec<char>> {
    let height = mat[0].len();
    let mut result = vec![vec![]; height];

    for v in mat {
        for (ri, row) in v.iter().enumerate() {
            for &cell in row {
                result[ri].push(cell);
            }
        }
    }

    result
}

fn combine_vertical(mat: [&Vec<Vec<char>>; 3]) -> Vec<Vec<char>> {
    let mut result = vec![];

    for v in mat {
        for row in v {
            result.push(row.clone());
        }
    }

    result
}
