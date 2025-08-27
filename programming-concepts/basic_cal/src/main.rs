/*
This project is a simple calculator that takes input in the form of "a+b", "a-b", "a*b", or "a/b",
where 'a' and 'b' are single-digit integers (0-9). It performs the specified arithmetic operation
and outputs the result. The calculator handles basic error checking, such as ensuring that the input
is in the correct format and that division by zero is avoided.
*/

use std::io;

fn main() {
    loop {
        let mut buffer = String::new();

        let input = io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line!");

        if input == 0 {
            println!("Empty input, please provide the calculation intended.")
        }

        let buffer = buffer.trim();
        if buffer.eq_ignore_ascii_case("exit") {
            println!("Closing program...");
            break;
        }

        println!("The result for '{}' is: {}", buffer, calc(buffer.chars()));
    }
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
