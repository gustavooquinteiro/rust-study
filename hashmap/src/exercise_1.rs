use std::collections::HashMap;

fn main() {
    let mut vector = Vec::new();
    vector.push(12);
    vector.push(32);
    vector.push(13);
    vector.push(14);
    vector.push(12);
    vector.push(42);
    vector.push(64);
    vector.push(25);
    vector.push(63);
    vector.sort();
    let mut mean = 0;
    let mut map = HashMap::new();
    for element in &vector {
        mean += element;
        let count = map.entry(element).or_insert(0);
        *count += 1;
    }
    let mut big = 0;
    let mut mode = 0;
    for (key, value) in map{
        if big < value {
            big = value;
            mode = *key;
        }
    }
    
    
    let median = vector[vector.len()/2];
    mean /= vector.len();
    println!("Media > {}\nMediana > {}\nModa > {}", mean, median, mode);
    
}
