use std::thread;
pub use std::sync::{mpsc, Mutex, Arc};
use std::cell::RefCell;
pub use crossbeam::atomic::AtomicCell;

const MAX_THREADS: usize = 4;

pub fn mergesort_st(arr: &mut [i32], left: usize, right: usize) {
    let size = right - left;
    let mut helper: Vec<i32> = vec![0; size];
    merge_st_helper(arr, left, right, &mut helper[..]);
}


fn merge_st(arr: &mut [i32], left: usize, mid: usize,right: usize, helper: &mut [i32])  {
    for i in left..mid {
        helper[i] = arr[i];
    }
    for i in mid..right {
        helper[i] = arr[i];
    }
    let mut left_ptr = left;
    let mut right_ptr = mid;
    let mut current_ptr = left;

    while left_ptr < mid && right_ptr < right {
        if helper[left_ptr] < helper[right_ptr] {
            arr[current_ptr] = helper[left_ptr];
            left_ptr += 1; 
        } else {
            arr[current_ptr] = helper[right_ptr];
            right_ptr += 1;
        }
        current_ptr += 1;
    }

    while left_ptr < mid {
        arr[current_ptr] = helper[left_ptr];
        left_ptr += 1;
        current_ptr += 1; 
    }

    while right_ptr < right {
        arr[current_ptr] = helper[right_ptr];
        right_ptr += 1;
        current_ptr += 1; 
    }
}

fn merge_st_helper(arr: &mut [i32], left: usize, right: usize, helper: &mut [i32]){
    if right - left > 1 {
        let mid = (left + right) / 2;

        merge_st_helper(arr, left, mid, helper);
        merge_st_helper(arr, mid, right, helper);

        merge_st(arr, left, mid, right, helper);
    }
}

///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////
pub fn mergesort_mt(arr:Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    return merge_mt_helper(arr, left, right, 0);
}

fn merge_mt_helper(arr: Vec<i32>, left: usize, right: usize, depth: usize) -> Vec<i32> {
    if right - left > 1 {
        let mid = (left + right) / 2;
        let new_depth = depth + 1;
        
        let (mut arr_left, mut arr_right) = match arr.split_at(mid) {
            (l, r) => (l.to_vec(), r.to_vec())
        };
        let arr_left_len = arr_left.len();
        let arr_right_len = arr_right.len();

        if new_depth < MAX_THREADS {
            let (sender, receiver) = mpsc::channel();
            let left_ptr = RefCell::new(arr_left);
            let _ = thread::spawn(move || {
                let left_sorted = merge_mt_helper(left_ptr.into_inner(), 0, arr_left_len, new_depth);
                sender.send(left_sorted).unwrap();
            });
            arr_right = merge_mt_helper(arr_right, 0, arr_right_len, new_depth);
            arr_left = receiver.recv().unwrap();
        } else {
            arr_left = merge_mt_helper(arr_left, 0, arr_left_len, new_depth);
            arr_right = merge_mt_helper(arr_right, 0, arr_right_len, new_depth);
        }
        return merge_mt(arr_left, arr_right);
    }
    return arr;
}



fn merge_mt(arr_left: Vec<i32>, arr_right: Vec<i32>) -> Vec<i32> {
    let arr_left_len = arr_left.len();
    let arr_right_len = arr_right.len();
    let mut arr_merged = Vec::with_capacity(arr_left_len + arr_right_len);
    
    let mut left_ptr = 0;
    let mut right_ptr = 0;
    
    while left_ptr < arr_left_len && right_ptr < arr_right_len {
        if arr_left[left_ptr] < arr_right[right_ptr] {
            arr_merged.push(arr_left[left_ptr]);
            left_ptr += 1;
        } else {
            arr_merged.push(arr_right[right_ptr]);
            right_ptr += 1;
        }
    }

    while left_ptr < arr_left_len {
        arr_merged.push(arr_left[left_ptr]);
        left_ptr += 1;
    }

    while right_ptr < arr_right_len {
        arr_merged.push(arr_right[right_ptr]);
        right_ptr += 1;
    }
    return arr_merged;
}



////////////////////////////////////////////////
///////////////////////////////////////////////////
///////////////////////////////////////////////////

pub fn mergesort_mt_mutex(arr: Arc<Mutex<Vec<i32>>>, left: usize, right: usize) {
    let size = right - left;
    merge_mt_helper_mutex(Arc::clone(&arr), left, right, 0);
}


fn merge_mt_helper_mutex(arr: Arc<Mutex<Vec<i32>>>, left: usize, right: usize, depth: usize) {
    if right - left > 1 {
        let mid = (left + right) / 2;
        let new_depth = depth + 1;
        
        if new_depth < MAX_THREADS {
            let arr_cloned1 = Arc::clone(&arr);
            let arr_cloned2 = Arc::clone(&arr);

            let handle1 = thread::spawn(move || {
                merge_mt_helper_mutex(arr_cloned1, left, mid, new_depth); 
            });
            

            let handle2 = thread::spawn(move || {
                merge_mt_helper_mutex(arr_cloned2, mid, right,new_depth); 
            });

            handle1.join().unwrap();
            handle2.join().unwrap();
        } else {
            merge_mt_helper_mutex(Arc::clone(&arr), left, mid, new_depth);
            merge_mt_helper_mutex(Arc::clone(&arr), mid, right, new_depth);
        }
        
        merge_mt_mutex(Arc::clone(&arr), left, mid, right);
    }
}

fn merge_mt_mutex(arr: Arc<Mutex<Vec<i32>>>, left: usize, mid: usize, right: usize) {
    let mut arr_ref = arr.lock().unwrap();
    let left_len = mid - left;
    let right_len = right - mid;
    let mut left_arr = Vec::with_capacity(left_len);
    let mut right_arr = Vec::with_capacity(right_len);
    for i in 0..left_len {
        left_arr.push(arr_ref[i + left]);
    }
    for i in 0..right_len {
        right_arr.push(arr_ref[i + mid]);
    }
    let mut left_ptr = 0;
    let mut right_ptr = 0; 
    let mut current_ptr = left;
    while left_ptr < left_len && right_ptr < right_len {
        if left_arr[left_ptr] < right_arr[right_ptr] {
            arr_ref[current_ptr] = left_arr[left_ptr];
            left_ptr += 1; 
        } else {
            arr_ref[current_ptr] = right_arr[right_ptr];
            right_ptr += 1;
        }
        current_ptr += 1;
    }
    
    while left_ptr < left_len {
        arr_ref[current_ptr] = left_arr[left_ptr];
        left_ptr += 1;
        current_ptr += 1; 
    }

    while right_ptr < right_len {
        arr_ref[current_ptr] = right_arr[right_ptr];
        right_ptr += 1;
        current_ptr += 1; 
    }
}







// pub fn mergesort_mt_forjon(arr: Arc<AtomicCell<Vec<i32>>>, left: usize, right: usize) {
//     let size = right - left;
//     let mut helper: Vec<i32> = vec![0; size];
//     let helper_atomic = Arc::new(AtomicCell::new(helper));
//     merge_mt_helper_forkjoin(Arc::clone(&arr), left, right, Arc::clone(&helper_atomic), 0);
// }

// fn merge_mt_helper_forkjoin(arr: Arc<AtomicCell<Vec<i32>>>, left: usize, right: usize, helper: Arc<AtomicCell<Vec<i32>>>, depth: usize) {
//     if right - left > 1 {
//         let mid = (left + right) / 2;
//         let new_depth = depth + 1;
        
        
//         if new_depth < MAX_THREADS {
//             let arr_atomic = Arc::clone(&arr);
//             let helper_atomic = Arc::clone(&helper);
//             let handle = thread::spawn(move || {
//                 merge_mt_helper_forkjoin(arr_atomic, left, mid, helper_atomic, new_depth); 
//             });
//             merge_mt_helper_forkjoin(Arc::clone(&arr), mid, right, Arc::clone(&helper), new_depth);
//             handle.join().unwrap();
//         } else {
//             merge_mt_helper_forkjoin(Arc::clone(&arr), left, mid, Arc::clone(&helper), new_depth);
//             merge_mt_helper_forkjoin(Arc::clone(&arr), mid, right, Arc::clone(&helper), new_depth);
//         }
//         merge_mt_forkfoin(Arc::clone(&arr) , left, mid, right, Arc::clone(&helper));
//     }
// }

// fn merge_mt_forkfoin(arr: Arc<AtomicCell<Vec<i32>>>, left: usize, mid: usize, right: usize, helper: Arc<AtomicCell<Vec<i32>>>) {
//     let arr_ref = arr.get_mut();
//     let helper_ref = helper.get_mut();
//     for i in left..mid {
//         helper_ref[i] = arr_ref[i];
//     }
//     for i in mid..right {
//         helper_ref[i] = arr_ref[i];
//     }
//     let mut left_ptr = left;
//     let mut right_ptr = mid;
//     let mut current_ptr = left;

//     while left_ptr < mid && right_ptr < right {
//         if helper_ref[left_ptr] < helper_ref[right_ptr] {
//             arr_ref[current_ptr] = helper_ref[left_ptr];
//             left_ptr += 1; 
//         } else {
//             arr_ref[current_ptr] = helper_ref[right_ptr];
//             right_ptr += 1;
//         }
//         current_ptr += 1;
//     }

//     while left_ptr < mid {
//         arr_ref[current_ptr] = helper_ref[left_ptr];
//         left_ptr += 1;
//         current_ptr += 1; 
//     }

//     while right_ptr < right {
//         arr_ref[current_ptr] = helper_ref[right_ptr];
//         right_ptr += 1;
//         current_ptr += 1; 
//     }
// }