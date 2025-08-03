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
    let mut res: u32;

    for i in 0..=n {
        if i <= 1 {
            res = i;
        } else {
            res = (i - 1) + (i - 2);
        }
        println!("{}", res);
    }
}

/*
fn fibonacci_two(n: u32) -> () {
    let c: u32 = n;
    let res: [u32; c] = [0; c];

    for i in 0..=n {
        if i <= 1 {
            res[i] = i;
        } else {
            res[i] = res[i-1] + res[i-2];
        }
    }

    println!("{:?}", res);
}
*/
