use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // Utilização de entry() para fazer o hashmap se comportar como um set
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(60);
    scores.entry(String::from("Red")).or_insert(40);
    println!("{:?}", scores);
}
