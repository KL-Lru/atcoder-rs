use proconio::input;

fn main() {
    input! {
      _: usize, k: usize,
      mut s: String,
    }

    s = replace_around_o(s);
    let run_length = to_run_length(&s);

    if current_o(&run_length) == k {
        println!("{}", maximize_dot(&run_length));
    } else if possible_o(&run_length) == k {
        println!("{}", maximize_o(&run_length));
    } else {
        println!("{s}");
    }
}

fn replace_around_o(s: String) -> String {
    let mut s = s.clone().chars().collect::<Vec<char>>();

    for i in 0..s.len() {
        if s[i] == 'o' {
            if i != 0 && s[i - 1] != 'o' {
                s[i - 1] = '.';
            }
            if i + 1 < s.len() && s[i + 1] != 'o' {
                s[i + 1] = '.';
            }
        }
    }

    s.iter().collect()
}

fn to_run_length(s: &str) -> Vec<(char, usize)> {
    s.chars().fold(vec![], |mut acc, c| {
        match acc.last_mut() {
            Some((lc, count)) if *lc == c => {
                *count += 1usize;
            }
            _ => acc.push((c, 1)),
        }
        acc
    })
}

fn maximize_o(run_length: &[(char, usize)]) -> String {
    run_length
        .iter()
        .map(|(c, count)| match c {
            '?' if count % 2 == 1 => vec!["o"; (count / 2) + 1].join("."),
            ch => ch.to_string().repeat(*count),
        })
        .collect::<Vec<String>>()
        .join("")
}

fn maximize_dot(run_length: &[(char, usize)]) -> String {
    run_length
        .iter()
        .map(|(c, count)| match c {
            '?' => ".".repeat(*count),
            ch => ch.to_string().repeat(*count),
        })
        .collect::<Vec<String>>()
        .join("")
}

fn current_o(run_length: &[(char, usize)]) -> usize {
    run_length
        .iter()
        .filter_map(|(c, count)| match c {
            'o' => Some(*count),
            _ => None,
        })
        .sum()
}

fn possible_o(run_length: &[(char, usize)]) -> usize {
    run_length
        .iter()
        .filter_map(|(c, count)| match c {
            'o' => Some(*count),
            '?' => Some((*count) / 2 + (*count) % 2),
            _ => None,
        })
        .sum()
}
