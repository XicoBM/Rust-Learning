use std::io;

/*
This is a program made to test my knowledge of common programming concepts using Rust.
It is a simple converter of temperatures: it converts Fahrenheit to Celsius and vice versa.
*/

fn main() {
    println!("Please select the input temperature type: Celsius or Fahrenheit");

    let mut temp_type = String::new();
    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to read line!");

    let temp_type = temp_type.trim();

    println!("Please input the respective temperature:");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line!");

    let num_temp: f64 = temp
        .trim()
        .parse()
        .expect("It was not possible to convert the temperature");

    let converted = if temp_type == "Celsius" {
        (num_temp * 9.0 / 5.0) + 32.0
    } else if temp_type == "Fahrenheit" {
        (num_temp - 32.0) * 5.0 / 9.0
    } else {
        close_with_error().expect("Error: invalid temperature type");
        return;
    };

    println!(
        "The converted temperature from {} is: {}",
        temp_type, converted
    );
}

fn close_with_error() -> Result<(), String> {
    Err(
        "Please try again, input the correct type for the temperature: Celsius or Fahrenheit"
            .to_string(),
    )
}
