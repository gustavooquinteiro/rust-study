use std::io;

fn partition(vec: &mut Vec<u32>, low: u32, high: u32) -> u32 {
    
    let mut low_index = low as usize;
    let high_index = (high-1) as usize;
    
    let pivot_index = high_index as usize;
    let pivot = vec[pivot_index];
    
    let mut i = low_index;
    
    while low_index < high_index {
        if vec[low_index] < pivot {
            vec.swap(i, low_index);
            i += 1;
        }
        low_index += 1;
    }
    if vec[high_index] < vec[i] {
        vec.swap(i, high_index);
    }
        
    i as u32
}


fn quicksort(vec: &mut Vec<u32>, low: u32, high: u32) -> &mut Vec<u32> {
    if low < high {
        let p = partition(vec, low, high);
        quicksort(vec, low, p - 1);
        quicksort(vec, p+1, high);
    }
    vec
}


fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Expect string");
    let n: u32 = n.trim().parse().expect("Expect number");
    
    let mut vec: Vec<u32> = Vec::new();
    for _ in 0..n {
        let mut number = String::new();
        io::stdin().read_line(&mut number).expect("Expect string");
        let number: u32 = number.trim().parse().expect("Expect number");
        
        vec.push(number);
    }
    
    let vec = quicksort(&mut vec, 0, n-1);
    println!("{:?}", vec);
    
}
