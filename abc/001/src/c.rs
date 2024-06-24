//! refs: https://atcoder.jp/contests/abc001/tasks/abc001_3

use proconio::input;

fn main() {
    input! {
        deg: i32,
        dis: i32,
    }

    let dir = match deg as f32 {
        d if (112.5..337.5).contains(&d) => "NNE",
        d if (337.5..562.5).contains(&d) => "NE",
        d if (562.5..787.5).contains(&d) => "ENE",
        d if (787.5..1012.5).contains(&d) => "E",
        d if (1012.5..1237.5).contains(&d) => "ESE",
        d if (1237.5..1462.5).contains(&d) => "SE",
        d if (1462.5..1687.5).contains(&d) => "SSE",
        d if (1687.5..1912.5).contains(&d) => "S",
        d if (1912.5..2137.5).contains(&d) => "SSW",
        d if (2137.5..2362.5).contains(&d) => "SW",
        d if (2362.5..2587.5).contains(&d) => "WSW",
        d if (2587.5..2812.5).contains(&d) => "W",
        d if (2812.5..3037.5).contains(&d) => "WNW",
        d if (3037.5..3262.5).contains(&d) => "NW",
        d if (3262.5..3487.5).contains(&d) => "NNW",
        _ => "N",
    };

    let w = match ((dis as f32) * 10.0 / 60.0).round() / 10.0 {
        d if (0.0..=0.2).contains(&d) => 0,
        d if (0.3..=1.5).contains(&d) => 1,
        d if (1.6..=3.3).contains(&d) => 2,
        d if (3.4..=5.4).contains(&d) => 3,
        d if (5.5..=7.9).contains(&d) => 4,
        d if (8.0..=10.7).contains(&d) => 5,
        d if (10.8..=13.8).contains(&d) => 6,
        d if (13.9..=17.1).contains(&d) => 7,
        d if (17.2..=20.7).contains(&d) => 8,
        d if (20.8..=24.4).contains(&d) => 9,
        d if (24.5..=28.4).contains(&d) => 10,
        d if (28.5..=32.6).contains(&d) => 11,
        _ => 12,
    };

    if w == 0 {
        println!("C {}", w);
    } else {
        println!("{} {}", dir, w);
    }
}
