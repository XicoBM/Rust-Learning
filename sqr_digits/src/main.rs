fn main() {
    let result = square_digits(123);
    println!("O resultado é: {}", result);
}

fn square_digits(num: u64) -> u64 {
    let var = num.to_string()
    .chars()
    .map(|c| {
        let digit = c.to_digit(10).unwrap();
        (digit*digit).to_string()
    })
    .collect::<String>()
    .parse()
    .unwrap();

    return var;
}