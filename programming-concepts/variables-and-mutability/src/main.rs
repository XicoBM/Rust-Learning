

fn array_testing() {
    let a = [3; 5];
    println!("{:?}", a);
}

fn shadowing_testing() {
    let x = 5; //1st use of the variabel "x"
    let x = x + 1; //2nd use of said variable

    {
        let x = x*2;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The final value of x is {x}");
}

fn shadowing_testing2() {
    let x = 10;
    let mut y = 1;

    loop {
        let b = x % y;

        if b == 0 && y != 1 && y % 2 != 0{
            break;
        } else {
            y = y + 1;
        }
    }

    println!("the inner y is {y}");
}

fn five() -> i32 {
	5
}

fn five_snd(x: i32) -> i32 {
	x + 1
}

fn main() {
	let x = five();
	let y = five_snd(1);
	println!("...{x}");
	println!("...{y}");
}
