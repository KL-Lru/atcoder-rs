use proconio::input;

fn main() {
    input! { d: String }

    let ans = match d.as_str() {
        "N" => "S",
        "S" => "N",
        "W" => "E",
        "E" => "W",
        "NE" => "SW",
        "SW" => "NE",
        "NW" => "SE",
        "SE" => "NW",
        _ => {
            panic!("unreachable")
        }
    };

    println!("{ans}");
}
