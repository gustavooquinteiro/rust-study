use std::io;

fn partition<T,F>(vec: &mut [T], f: &F) -> usize 
    where F: Fn(&T,&T) -> bool 
{
    
    let len = vec.len();
    let last_index = len - 1;
        
    let mut store_index = 0;
    
    for i in 0..last_index {
        if f(&vec[i], &vec[last_index]) {
            vec.swap(i, store_index);
            store_index += 1;
        }
    }
    vec.swap(store_index, last_index);
    store_index
}


fn quicksort<T,F>(vec: &mut [T], f: &F) 
    where F: Fn(&T, &T) -> bool
{
    let len = vec.len();
    if len >= 2 {
        let p = partition(vec, f);
        quicksort(&mut vec[0..p], f);
        quicksort(&mut vec[p+1..len], f);
    }
}

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
    
    quicksort(&mut vec, &|x, y| x < y);
    println!("{:?}", vec);
    
}
