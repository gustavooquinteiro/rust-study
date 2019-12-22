use std::io;

fn fibonacci(x: u32) -> u32 {
    if (x == 1)  | (x == 2 ){
        1
    } else {
        fibonacci(x-1) + fibonacci(x-2)
    }
}

fn main() {
    println!("Insira um numero: ");
    let mut number = String::new();
    io::stdin().read_line(&mut number).expect("Failed to read");
    let number: u32 = number.trim().parse().expect("Failed to parse");
    println!("{}º numero Fibonacci = {}", number, fibonacci(number));    
}
