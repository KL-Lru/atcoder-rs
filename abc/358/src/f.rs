use std::cmp::min;

use proconio::input;

// `+++++ ... +++S+`
fn draw_start_line(m: i32) {
    println!("{}S+", "+".repeat(((2 * m) - 1) as usize));
}

// `+++++ ... +++G+`
fn draw_goal_line(m: i32) {
    println!("{}G+", "+".repeat(((2 * m) - 1) as usize));
}

// `+-+-+ ... +-+.+`
fn draw_last_pass_wall(m: i32) {
    println!("{}+.+", "+-".repeat((m - 1) as usize));
}

// `+.+-+ ... +-+-+`
fn draw_first_pass_wall(m: i32) {
    println!("+.{}+", "+-".repeat((m - 1) as usize));
}

// `+o|o|o| ... |o+`
fn draw_vertical_straight(m: i32) {
    println!("+o{}+", "|o".repeat((m - 1) as usize));
}

// `+o.o.o. ... .o+`
fn draw_horizontal_straight(m: i32) {
    println!("+o{}+", ".o".repeat((m - 1) as usize));
}

fn draw_two_lines(m: i32, remain: i32, last_line: bool) -> i32 {
    let consume = min(m - 1, remain / 2);

    let mut path_str = "".to_string();
    let mut wall_str = "".to_string();

    for idx in 0..(2 * m + 1) {
        path_str += if idx == 0 || idx == 2 * m {
            "+"
        } else if idx % 2 == 1 {
            "o"
        } else if (idx / 2) <= consume {
            "."
        } else {
            "|"
        };
    }

    for idx in 0..(2 * m + 1) {
        wall_str += if idx == 0 || idx == 2 * m || idx % 2 == 0 {
            "+"
        } else if (idx / 2) == consume {
            "."
        } else {
            "-"
        };
    }

    println!("{}", path_str.chars().rev().collect::<String>());
    println!("{}", wall_str.chars().rev().collect::<String>());
    println!("{}", path_str.chars().rev().collect::<String>());

    if !last_line {
        draw_last_pass_wall(m);
    }

    remain - consume * 2
}

fn draw_last_three_line(m: i32, remain: i32) {
    let consume = (remain - 2 * (m - 1)) / 2;

    let mut path_str = "+o|".to_string();
    let mut last_path_str = "+o.".to_string();
    let mut wall_str = "+.+".to_string();

    draw_horizontal_straight(m);
    draw_first_pass_wall(m);

    for _ in 0..consume {
        if path_str.len() < ((2 * m + 1) - 4) as usize {
            path_str += "o.o|";
            last_path_str += "o|o.";
            wall_str += ".+.+";
        } else {
            path_str += "o.o+";
            last_path_str += "o|o+";
            wall_str += ".+.+";
        }
    }

    while path_str.len() < (2 * m + 1) as usize {
        if path_str.len() < (2 * m - 1) as usize {
            path_str += "o|";
        } else {
            path_str += "o+";
        }
    }
    while last_path_str.len() < (2 * m + 1) as usize {
        if last_path_str.len() < (2 * m - 1) as usize {
            last_path_str += "o.";
        } else {
            last_path_str += "o+";
        }
    }
    while wall_str.len() < (2 * m + 1) as usize {
        wall_str += "-+";
    }

    println!("{}", path_str);
    println!("{}", wall_str);
    println!("{}", last_path_str);
}

fn draw_even_grid(n: i32, m: i32, k: i32) {
    let mut remain = k - n;

    for idx in 0..(n / 2) {
        remain = draw_two_lines(m, remain, idx == (n / 2 - 1));
    }
}

fn draw_odd_grid(n: i32, m: i32, k: i32) {
    let mut remain = k - n;

    for _ in 0..((n - 3) / 2) {
        remain = draw_two_lines(m, remain, false);
    }
    if remain <= 2 * (m - 1) {
        draw_two_lines(m, remain, false);
        draw_vertical_straight(m);
    } else {
        draw_last_three_line(m, remain);
    }
}

fn main() {
    input! {
      n: i32,
      m: i32,
      k: i32,
    }

    if k < n || (k - n) % 2 != 0 {
        println!("No");
        return;
    }

    println!("Yes");

    draw_start_line(m);
    if n % 2 == 0 {
        draw_even_grid(n, m, k);
    } else {
        draw_odd_grid(n, m, k)
    }
    draw_goal_line(m);
}
