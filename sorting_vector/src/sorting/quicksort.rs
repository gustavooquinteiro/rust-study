fn partition<T, F>(vec: &mut [T], f: &F) -> usize
where
    F: Fn(&T, &T) -> bool,
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

pub fn quicksort<T, F>(vec: &mut [T], f: &F)
where
    F: Fn(&T, &T) -> bool,
{
    let len = vec.len();
    if len >= 2 {
        let p = partition(vec, f);
        quicksort(&mut vec[0..p], f);
        quicksort(&mut vec[p + 1..len], f);
    }
}
