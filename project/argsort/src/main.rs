// extern crate ndarray;
// use ndarray::prelude::*;

fn main() {
    let mut arr = vec![6, 5, 4, 1, 2, 3, 4];
    let high = arr.len() - 1;
    let mut argarr = vec![0, 1, 2, 3, 4, 5, 6];
    quick_sort(&mut arr[..], 0, high, &mut argarr[..]);
    println!("Arr: {:?}", arr);
    println!("Argarr: {:?}", argarr);
}

pub fn quick_sort(arr: &mut [i32], low: usize, high: usize, argarr: &mut [usize]) {
    if low < high {
        let pi = partition(arr, low, high, argarr);
        quick_sort(arr, low, pi - 1, argarr);
        quick_sort(arr, pi + 1, high, argarr);
    }
}

pub fn partition(arr: &mut [i32], low: usize, high: usize, argarr: &mut [usize]) -> usize {
    let pivot = arr[high];

    let mut i: usize = low;
    
    for j in low..high {
        if arr[j] < pivot {
            swap(arr, i, j);
            swap(argarr, i, j);
            i+=1;
        }
    }
    swap(arr, i, high);
    swap(argarr, i, high);
    return i;
}

pub fn swap<T: Copy>(arr: &mut [T], idx1: usize, idx2: usize) {
    let temp = arr[idx1];
    arr[idx1] = arr[idx2];
    arr[idx2] = temp;
}




// pub fn argsort(arr: &mut [i32]) -> Vec<usize> {
//     let size = arr.len();
//     let mut res = Vec::with_capacity(size);
    

// }

// pub fn argsort(all_similarity: &ArrayView<f64, Ix1>, mut similarity: ArrayViewMut<f64, Ix1>) {
// every row of  similarity matrix is filled with initial index 0, 1, 2, .....
// }