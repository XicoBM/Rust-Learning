
fn main() {
    println!("6 é quadrado perfeito? {}", perfect_squares2(6));
    println!("9 é quadrado perfeito? {}", perfect_squares2(9));
    println!("16 é quadrado perfeito? {}", perfect_squares2(16));
    println!("15 é quadrado perfeito? {}", perfect_squares2(15));

    println!("6 é quadrado perfeito? {}", perfect_squares(6));
    println!("9 é quadrado perfeito? {}", perfect_squares(9));
    println!("16 é quadrado perfeito? {}", perfect_squares(16));
    println!("15 é quadrado perfeito? {}", perfect_squares(15));
}

fn perfect_squares(n: i64) -> bool {
    if n < 0 {
        return false;
    }

    if n == 0 {
        return true;
    }
    
    let mut divisor_count = 0;
    
    for x in 1..=n {
        if n % x == 0 {
            divisor_count += 1;
        }
    }
    
    let res: bool = divisor_count % 2 == 1;
    return res;
}

fn perfect_squares2(n: i64) -> bool {
    if n < 0 {
        return false;
    }
    let sqrt_n = (n as f64).sqrt() as i64;
    
    sqrt_n * sqrt_n == n
}