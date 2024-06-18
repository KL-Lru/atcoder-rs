//! refs: https://atcoder.jp/contests/abc001/tasks/abc001_4

use proconio::input;

fn time_to_index(num: i32) -> usize {
    let hour = num / 100;
    let minute = num % 100;
    (hour * 12 + minute / 5) as usize
}

fn index_to_time(num: usize) -> i32 {
    let hour = num / 12;
    let minute = num * 5 % 60;
    (hour * 100 + minute) as i32
}

fn parse_time_range(time_str: String) -> (i32, i32) {
    let parsed_vec: Vec<String> = time_str
        .split('-')
        .collect::<Vec<&str>>()
        .iter()
        .map(|s| s.to_string())
        .collect();

    return (
        floor_round(parse_int(parsed_vec.first().cloned())),
        ceil_round(parse_int(parsed_vec.last().cloned())),
    );
}

fn parse_int(s: Option<impl Into<String>>) -> i32 {
    if let Some(num_str) = s {
        if let Ok(n) = num_str.into().parse::<i32>() {
            return n;
        }
    }
    panic!("Parse Error");
}

fn floor_round(num: i32) -> i32 {
    num - num % 5
}

fn ceil_round(num: i32) -> i32 {
    num + (5 - num % 5) % 5
}

fn accumulate(sequence: &mut [i32]) {
    for i in 1..sequence.len() {
        sequence[i] += sequence[i - 1];
    }
}

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut sequence = [0; 24 * 12 + 1];
    for str in s {
        let (start, end) = parse_time_range(str);

        sequence[time_to_index(start)] += 1;
        sequence[time_to_index(end)] -= 1;
    }
    accumulate(sequence.as_mut());

    let mut results = Vec::<String>::new();
    let mut check_point = -1;
    for (i, v) in sequence.iter().enumerate().take(12 * 24 + 1) {
        if *v > 0 && check_point == -1 {
            check_point = index_to_time(i);
        } else if *v <= 0 && check_point != -1 {
            results.push(format!("{:04}-{:04}", check_point, index_to_time(i)));
            check_point = -1;
        }
    }
    if check_point != -1 {
        results.push(format!("{:04}-{:04}", check_point, 2400));
    }

    for result in results {
        println!("{}", result);
    }
}
