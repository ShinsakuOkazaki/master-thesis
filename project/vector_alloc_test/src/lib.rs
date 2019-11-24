#[inline]
pub fn vector_addition(initial_capacity: usize, final_capacity: usize) {
    let mut vector = Vec::with_capacity(initial_capacity);
    for i in 0..final_capacity {
        vector.push(i);
    }
}
