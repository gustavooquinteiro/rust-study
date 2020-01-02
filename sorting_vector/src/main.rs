extern crate sorting_vector;
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Expect string");
    let n: usize = n.trim().parse().expect("Expect number");

    let mut vec: Vec<u32> = Vec::new();
    for _ in 0..n {
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Expect string");
        let number: u32 = number.trim().parse().expect("Expect number");

        vec.push(number);
    }

    sorting_vector::sorting::mergesort::mergesort(&mut vec, &|x, y| x < y);
    println!("{:?}", vec);
}
