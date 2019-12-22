use std::io;

fn greedy_fibonacci(x: u32) -> u32 {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(1);
    for i in 2..x as usize{
        vec.push(vec[i-1] + vec[i-2]);
    }
    vec[(x-1) as usize]
}

fn fibonacci(x: u32) -> u32 {
    if (x == 1)  | (x == 2 ){
        1
    } else {
        fibonacci(x-1) + fibonacci(x-2)
    }
}

fn main() {
    loop {
        println!("Insira um numero (0 para sair): ");
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Failed to read");
        let number: u32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if number == 0 {
            break;
        }
    
        println!("{}º numero Fibonacci = {}", number, greedy_fibonacci(number));    
    }
}
