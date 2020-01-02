fn merge<T, F>(right: &[T], left: &[T], ret: &mut [T], f: &F)
where
    F: Fn(&T, &T) -> bool,
    T: std::marker::Copy,
{
    let mut left_index = 0;
    let mut right_index = 0;
    let mut index = 0;

    while (right_index < right.len()) & (left_index < left.len()) {
        if f(&left[left_index], &right[right_index]) {
            ret[index] = left[left_index];
            left_index += 1;
        } else {
            ret[index] = right[right_index];
            right_index += 1;
        }
        index += 1;
    }

    if left_index < left.len() {
        ret[index..].copy_from_slice(&left[left_index..]);
    }
    if right_index < right.len() {
        ret[index..].copy_from_slice(&right[right_index..]);
    }
}

pub fn mergesort<T, F>(vec: &mut [T], f: &F)
where
    F: Fn(&T, &T) -> bool,
    T: std::marker::Copy,
{
    let middle = vec.len() / 2;
    if middle != 0 {
        mergesort(&mut vec[..middle], f);
        mergesort(&mut vec[middle..], f);

        let mut array = vec.to_vec();
        merge(&vec[middle..], &vec[..middle], &mut array[..], f);

        vec.copy_from_slice(&array);
    }
}
