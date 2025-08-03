use std::io;

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("It was not possible to read the input. Please try again.");

    let var: u32 = input
        .trim()
        .parse()
        .expect("It was not possible to complete the process. Please try again.");

    fibonacci(var);
}

fn fibonacci(n: u32) -> () {
    let res: u32;

    if n == 0 {
        res = 0;
    } else if n == 1 {
        res = 1;
    } else {
        res = (n - 1) + (n - 2);
    }

    println!("{}", res);
}
