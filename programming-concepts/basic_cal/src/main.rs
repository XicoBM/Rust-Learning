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

        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line!");

        let buffer = buffer.trim();
        if buffer.eq_ignore_ascii_case("exit") {
            println!("Closing program...");
            break;
        }

        println!("The result for '{}' is: {}", buffer, calc(buffer.chars()));
    }
}
/*
fn input_analysis (input: impl Iterator<Item = char>, index: u32) -> char {
    let chars: Vec<char> = input.collect();

    if chars.len() < 3 {
        return 0;
    }

    let mut index = index;
    let mut var_one: char = '';
    let mut var_two: char = '';
    let mut var = '';
    loop {
        var = chars[index].to_digit(10).unwrap_or(0);

        if var == 0 {
            return var_final;
            break;
        } else {
            var_final = var_final.concat!(var);
            index += 1;
        }
    }
}
*/

fn calc(input: impl Iterator<Item = char>) -> usize {
    let mut elem1: String = "".to_string();
    let mut elem2: String = "".to_string();
    let mut operation: char = ' ';
    let mut var_temp: usize = 0;
    let input: Vec<char> = input.collect();

    for n in 0..(input.len()) {
        if input[n].to_digit(10) == None {
            operation = input[n];
            break;
        }
        elem1.push(input[n]);
        var_temp = n+2;
    }

    for nn in var_temp..(input.len()) {
        if input[nn].to_digit(10) == None {
            break;
        }
        elem2.push(input[nn]);
    }

    let elem1: usize = elem1.parse().unwrap();
    let elem2: usize = elem2.parse().unwrap();

    match operation {
        '*' => elem1 * elem2,
        '+' => elem1 + elem2,
        '-' => elem1 - elem2,
        '/' => {
            if elem2 == 0 {
                0
            } else {
                elem1 / elem2
            }
        }
        _ => 0,
    }
}
