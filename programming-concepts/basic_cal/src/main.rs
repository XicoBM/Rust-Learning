use std::io;

fn main() {
    let mut calc_in = String::new();
    io::stdin()
        .read_line(&mut calc_in)
        .expect("Failed to read line!");

    let calc_in = calc_in.trim().chars();

    let res: i64 = calc(calc_in);
    println!("{}", res);
}

fn calc(input: impl Iterator<Item = char>) -> i64 {
    let chars: Vec<char> = input.collect();

    if chars.len() < 3 {
        return 0;
    }

    let elem1 = chars[0].to_digit(10).unwrap_or(0) as i64;
    let operation = chars[1];
    let elem3 = chars[2].to_digit(10).unwrap_or(0) as i64;

    match operation {
        '*' => elem1 * elem3,
        '+' => elem1 + elem3,
        '-' => elem1 - elem3,
        '/' => {
            if elem3 == 0 {
                0
            } else {
                elem1 / elem3
            }
        }
        _ => 0,
    }
}
