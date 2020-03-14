use std::thread;
pub use std::sync::{mpsc, Mutex, Arc};
use std::cell::RefCell;
pub use crossbeam::atomic::AtomicCell;
use crate::objects::customer::*;
use std::marker::{Send, Sync};
use std::mem;
use std::collections::{LinkedList, VecDeque};
use std::default::Default;

const MAX_THREADS: usize = 4;

pub fn mergesort_vecdeque_slice<T: 'static>(mut arr: Vec<T>, len: usize) -> VecDeque<T> 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    let res = merge_helper_vecdeque_slice(&mut arr[..], len, 0);
    return res;
}


fn merge_helper_vecdeque_slice<T: 'static>(arr: &mut [T], len: usize ,depth: usize) -> VecDeque<T> 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    if len > 1 {
        let mid = len / 2;
        let new_depth = depth + 1;
        let (arr_left, arr_right) = arr.split_at_mut(mid);
        let len_left = arr_left.len();
        let len_right = arr_right.len();
        let res_left;
        let res_right;
        if new_depth < MAX_THREADS {
            let (left_thread, right_thread) = crossbeam::scope(|scope| {
                
                let left_handler = scope.spawn(move |_| {
                    merge_helper_vecdeque_slice(arr_left, len_left, new_depth)
                });
                let right_handler = scope.spawn(move |_| {
                    merge_helper_vecdeque_slice(arr_right, len_right, new_depth)
                });
                
                let l = left_handler.join().unwrap();
                let r = right_handler.join().unwrap();
                (l, r)
            }).unwrap();
            res_left = left_thread;
            res_right = right_thread;
        } else {
            res_left = merge_helper_vecdeque_slice(arr_left, len_left ,new_depth);
            res_right = merge_helper_vecdeque_slice(arr_right, len_right, new_depth);
        }
        return merge_vecdeque_slice(res_left, res_right);
    }
    return merge_vecdeque_base_slice(arr);
}

fn merge_vecdeque_slice<T: 'static>(mut arr_left: VecDeque<T>, mut arr_right: VecDeque<T>) -> VecDeque<T> 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    let arr_left_len = arr_left.len();
    let arr_right_len = arr_right.len();
    let mut arr_merged = VecDeque::with_capacity(arr_left_len + arr_right_len);

    while !arr_left.is_empty() && !arr_right.is_empty() {
        if arr_left.front().unwrap() < arr_right.front().unwrap() {
            arr_merged.push_back(arr_left.pop_front().unwrap());
        } else {
            arr_merged.push_back(arr_right.pop_front().unwrap());
        }
    }

    if !arr_left.is_empty() {
        arr_merged.append(&mut arr_left);
    }

    if !arr_right.is_empty() {
        arr_merged.append(&mut arr_right);
    }

    return arr_merged;
}

fn merge_vecdeque_base_slice<T: 'static>(arr: &mut [T]) -> VecDeque<T> 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    let mut arr_merged = VecDeque::with_capacity(1);
    let temp = mem::replace(&mut arr[0], T::default());
    arr_merged.push_back(temp);
    return arr_merged;
}

/////////////////////////////
////////////////////////////
///////////////////////////////
/////////////////////////////// 
pub fn mergesort_vecdeque<T: 'static>(mut arr:VecDeque<Arc<T>>, left: usize, right: usize) -> VecDeque<Arc<T>> 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    let l = arr.len();
    return merge_helper_vecdeque(Arc::new(arr), 0, l, 0);
}


fn merge_helper_vecdeque<T: 'static>(arr: Arc<VecDeque<Arc<T>>>, left: usize, right: usize, depth: usize) -> VecDeque<Arc<T>>  
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    if right - left > 1 {
        let mid = (left + right) / 2;
        let new_depth = depth + 1;
        let arr_cloned1 = Arc::clone(&arr);
        let arr_cloned2 = Arc::clone(&arr);
        let mut arr_right;
        let mut arr_left;

        if new_depth < MAX_THREADS {
            let (sender1, receiver1) = crossbeam::channel::unbounded();
            let (sender2, receiver2) = crossbeam::channel::unbounded(); 
            let _ = thread::spawn(move || {
                let sorted = merge_helper_vecdeque(arr_cloned1, left, mid, new_depth);
                sender1.send(sorted).unwrap();
            });
            let _ = thread::spawn(move || {
                let sorted = merge_helper_vecdeque(arr_cloned2, mid, right, new_depth);
                sender2.send(sorted).unwrap();
            });
            arr_left = receiver1.recv().unwrap();
            arr_right = receiver2.recv().unwrap(); 
        } else {
            arr_left = merge_helper_vecdeque(arr_cloned1, left, mid, new_depth);
            arr_right = merge_helper_vecdeque(arr_cloned2, mid, right, new_depth);
        }
        return merge_vecdeque(arr_left, arr_right);
    }
    return merge_vecdeque_base(arr, left);
}

fn merge_vecdeque<T: 'static>(mut arr_left: VecDeque<Arc<T>>, mut arr_right: VecDeque<Arc<T>>) -> VecDeque<Arc<T>> 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    let arr_left_len = arr_left.len();
    let arr_right_len = arr_right.len();
    let mut arr_merged = VecDeque::with_capacity(arr_left_len + arr_right_len);

    while !arr_left.is_empty() && !arr_right.is_empty() {
        if arr_left.front().unwrap() < arr_right.front().unwrap() {
            arr_merged.push_back(arr_left.pop_front().unwrap());
        } else {
            arr_merged.push_back(arr_right.pop_front().unwrap());
        }
    }

    if !arr_left.is_empty() {
        arr_merged.append(&mut arr_left);
    }

    if !arr_right.is_empty() {
        arr_merged.append(&mut arr_right);
    }

    return arr_merged;
}

fn merge_vecdeque_base<T: 'static>(arr: Arc<VecDeque<Arc<T>>>, left: usize) -> VecDeque<Arc<T>> 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    let mut arr_merged = VecDeque::with_capacity(1);
    arr_merged.push_back(Arc::clone(&arr[left]));
    return arr_merged;
}


/////////////////////////////////////
/////////////////////////////////////
/////////////////////////////////////
/////////////////////////////////////

pub fn mergesort_linkedlist<T: 'static>(mut head: LinkedList<T>) ->LinkedList<T> 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    let l = head.len();
    return merge_helper_linkedlist(head, 0, l, 0);
}

fn merge_helper_linkedlist<T: 'static>(mut head: LinkedList<T>, left: usize, right: usize, depth: usize) ->LinkedList<T> 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    if right - left > 1 {
        let mid = (left + right) / 2;
        let new_depth = depth + 1;
        let mut right_head = head.split_off(mid);
        let mut left_head = head;
        let right_len = right_head.len();
        let left_len = left_head.len();
        if new_depth < MAX_THREADS {
            let (sender, receiver) = mpsc::channel();
            let left_ptr = RefCell::new(left_head);
            let _ = thread::spawn(move || {
                let sorted = merge_helper_linkedlist(left_ptr.into_inner(), 0, left_len, new_depth);
                sender.send(sorted).unwrap();
            });
            right_head = merge_helper_linkedlist(right_head, 0, right_len, new_depth);
            left_head = receiver.recv().unwrap();
        } else {
            left_head = merge_helper_linkedlist(left_head, 0, left_len, new_depth);
            right_head = merge_helper_linkedlist(right_head, 0, right_len, new_depth);
        }
        return merge_linkedlist(left_head, right_head);
    }
    return head;
}

fn merge_linkedlist<T: 'static>(mut left_head: LinkedList<T>, mut right_head: LinkedList<T>) ->LinkedList<T> 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{    
    let mut merged = LinkedList::new();
    while !left_head.is_empty() && !right_head.is_empty() {
        if left_head.front().unwrap() < right_head.front().unwrap() {
            merged.push_back(left_head.pop_front().unwrap());
        } else {
            merged.push_back(right_head.pop_front().unwrap());
        }
    }

    if !left_head.is_empty() {
        merged.append(&mut left_head);
    }

    if !right_head.is_empty() {
        merged.append(&mut right_head);
    }
    return merged;
}

////////////////////////////////////////
////////////////////////////////////////
///////////////////////////////////////////
pub fn mergesort_mt_mp_gr_nocp<T: 'static>(mut arr:Vec<Arc<T>>, left: usize, right: usize) -> Vec<Arc<T>>
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    return merge_mt_helper_mp_gr_nocp(Arc::new(arr), left, right, 0);
}

fn merge_mt_helper_mp_gr_nocp<T: 'static>(arr: Arc<Vec<Arc<T>>>, left: usize, right: usize, depth: usize) -> Vec<Arc<T>>
    where T: Clone + Customer + PartialOrd + Send + Sync
{
    if right - left > 1 {
        let mid = (left + right) / 2;
        let new_depth = depth + 1;
        let arr_cloned1 = Arc::clone(&arr);
        let arr_cloned2 = Arc::clone(&arr);
        let mut arr_right;
        let mut arr_left;
        if new_depth < MAX_THREADS {
            let (sender, receiver) = mpsc::channel();
            let _ = thread::spawn(move || {
                let left_sorted = merge_mt_helper_mp_gr_nocp(arr_cloned1, left, mid, new_depth);
                sender.send(left_sorted).unwrap();
            });
            arr_right = merge_mt_helper_mp_gr_nocp(arr_cloned2, mid, right, new_depth);
            arr_left = receiver.recv().unwrap();
        } else {
            arr_left = merge_mt_helper_mp_gr_nocp(arr_cloned1, left, mid, new_depth);
            arr_right = merge_mt_helper_mp_gr_nocp(arr_cloned2, mid, right, new_depth);
        }
        return merge_mt_mp_gr_nocp(arr_left, arr_right);
    }
    return merge_mt_mp_base_gr_nocp(arr, left);
}

fn merge_mt_mp_gr_nocp<T: 'static>(mut arr_left: Vec<Arc<T>>, mut arr_right: Vec<Arc<T>>) -> Vec<Arc<T>>
    where T: Clone + Customer + PartialOrd + Send + Sync
{
    let arr_left_len = arr_left.len();
    let arr_right_len = arr_right.len();
    let mut arr_merged = Vec::with_capacity(arr_left_len + arr_right_len);
    
    let mut left_ptr = 0;
    let mut right_ptr = 0;
    
    while left_ptr < arr_left_len && right_ptr < arr_right_len {
        if arr_left[0] < arr_right[0] {
            arr_merged.push(arr_left.remove(0));
            left_ptr += 1;
        } else {
            arr_merged.push(arr_right.remove(0));
            right_ptr += 1;
        }
    }

    while left_ptr < arr_left_len {
        arr_merged.push(arr_left.remove(0));
        left_ptr += 1;
    }

    while right_ptr < arr_right_len {
        arr_merged.push(arr_right.remove(0));
        right_ptr += 1;
    }
    return arr_merged;
}

fn merge_mt_mp_base_gr_nocp<T>(arr: Arc<Vec<Arc<T>>>, left: usize) -> Vec<Arc<T>>
    where T: Clone + Customer + PartialOrd + Send + Sync
{
    let mut arr_merged = Vec::with_capacity(1);
    arr_merged.push(Arc::clone(&arr[left]));
    return arr_merged;
}



///////////////////////////////
///////////////////////////////
/////////////////////////////
pub fn mergesort_mt_mp_gemeric<T: 'static>(arr:Vec<T>, left: usize, right: usize) -> Vec<T> 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    return merge_mt_helper_mp_generic(Arc::new(arr), left, right, 0);
}

fn merge_mt_helper_mp_generic<T: 'static>(arr: Arc<Vec<T>>, left: usize, right: usize, depth: usize) -> Vec<T> 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    if right - left > 1 {
        let mid = (left + right) / 2;
        let new_depth = depth + 1;
        let arr_cloned1 = Arc::clone(&arr);
        let arr_cloned2 = Arc::clone(&arr);
        let mut arr_right;
        let mut arr_left;
        if new_depth < MAX_THREADS {
            let (sender, receiver) = mpsc::channel();
            let _ = thread::spawn(move || {
                let left_sorted = merge_mt_helper_mp_generic(arr_cloned1, left, mid, new_depth);
                sender.send(left_sorted).unwrap();
            });
            arr_right = merge_mt_helper_mp_generic(arr_cloned2, mid, right, new_depth);
            arr_left = receiver.recv().unwrap();
        } else {
            arr_left = merge_mt_helper_mp_generic(arr_cloned1, left, mid, new_depth);
            arr_right = merge_mt_helper_mp_generic(arr_cloned2, mid, right, new_depth);
        }
        return merge_mt_mp_generic(arr_left, arr_right);
    }
    return merge_mt_mp_base_generic(arr, left);
}

fn merge_mt_mp_generic<T: 'static>(mut arr_left: Vec<T>, mut arr_right: Vec<T>) -> Vec<T> 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    let arr_left_len = arr_left.len();
    let arr_right_len = arr_right.len();
    let mut arr_merged = Vec::with_capacity(arr_left_len + arr_right_len);
    
    let mut left_ptr = 0;
    let mut right_ptr = 0;
    
    while left_ptr < arr_left_len && right_ptr < arr_right_len {
        if arr_left[0] < arr_right[0] {
            arr_merged.push(arr_left.remove(0));
            left_ptr += 1;
        } else {
            arr_merged.push(arr_right.remove(0));
            right_ptr += 1;
        }
    }

    while left_ptr < arr_left_len {
        arr_merged.push(arr_left.remove(0));
        left_ptr += 1;
    }

    while right_ptr < arr_right_len {
        arr_merged.push(arr_right.remove(0));
        right_ptr += 1;
    }
    return arr_merged;
}

fn merge_mt_mp_base_generic<T>(arr: Arc<Vec<T>>, left: usize) -> Vec<T> 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    let mut arr_merged = Vec::with_capacity(1);
    arr_merged.push(arr[left].clone());
    return arr_merged;
}




//////////////////////////////////////
//////////////////////////////////////
/////////////////////////////////////////
///////////////////////////////////////// 


pub fn mergesort_mt_mp_improve(arr:Vec<i32>, left: usize, right: usize) -> Arc<Vec<i32>> {
    return merge_mt_helper_mp_improve(Arc::new(arr), left, right, 0);
}

fn merge_mt_helper_mp_improve(arr: Arc<Vec<i32>>, left: usize, right: usize, depth: usize) -> Arc<Vec<i32>> {
    if right - left > 1 {
        let mid = (left + right) / 2;
        let new_depth = depth + 1;
        let arr_cloned1 = Arc::clone(&arr);
        let arr_cloned2 = Arc::clone(&arr);
        let mut arr_right;
        let mut arr_left;
        if new_depth < MAX_THREADS {
            let (sender, receiver) = mpsc::channel();
            let _ = thread::spawn(move || {
                let left_sorted = merge_mt_helper_mp_improve(arr_cloned1, left, mid, new_depth);
                sender.send(left_sorted).unwrap();
            });
            arr_right = merge_mt_helper_mp_improve(arr_cloned2, mid, right, new_depth);
            arr_left = receiver.recv().unwrap();
        } else {
            arr_left = merge_mt_helper_mp_improve(arr_cloned1, left, mid, new_depth);
            arr_right = merge_mt_helper_mp_improve(arr_cloned2, mid, right, new_depth);
        }
        return merge_mt_mp_improve(arr_left, arr_right);
    }
    return merge_mt_mp_base_improve(arr, left);
}

fn merge_mt_mp_improve(arr_left: Arc<Vec<i32>>, arr_right: Arc<Vec<i32>>) -> Arc<Vec<i32>> {
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
    return Arc::new(arr_merged);
}

fn merge_mt_mp_base_improve(arr: Arc<Vec<i32>>, left: usize) -> Arc<Vec<i32>> {
    let mut arr_merged = Vec::with_capacity(1);
    arr_merged.push(arr[left]);
    return Arc::new(arr_merged);
}



///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////
///////////////////////////////////////////////////////////////
pub fn mergesort_mt_mp(arr:Vec<i32>, left: usize, right: usize) -> Vec<i32> {
    return merge_mt_helper_mp(arr, left, right, 0);
}

fn merge_mt_helper_mp(arr: Vec<i32>, left: usize, right: usize, depth: usize) -> Vec<i32> {
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
                let left_sorted = merge_mt_helper_mp(left_ptr.into_inner(), 0, arr_left_len, new_depth);
                sender.send(left_sorted).unwrap();
            });
            arr_right = merge_mt_helper_mp(arr_right, 0, arr_right_len, new_depth);
            arr_left = receiver.recv().unwrap();
        } else {
            arr_left = merge_mt_helper_mp(arr_left, 0, arr_left_len, new_depth);
            arr_right = merge_mt_helper_mp(arr_right, 0, arr_right_len, new_depth);
        }
        return merge_mt_mp(arr_left, arr_right);
    }
    return arr;
}



fn merge_mt_mp(arr_left: Vec<i32>, arr_right: Vec<i32>) -> Vec<i32> {
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



///////////////////////////////////////////
////////////////////////////////////////////////
//////////////////////////////////////////////
//////////////////////////////////////////////

pub fn mergesort_mt_im(arr: Arc<Vec<i32>>, left: usize, right: usize) -> Arc<Mutex<Vec<i32>>> {
    let res = Arc::new(Mutex::new(Vec::with_capacity(right - left)));
    merge_mt_helper_im(Arc::clone(&arr), left, right, Arc::clone(&res), 0);
    return res;
}

fn merge_mt_helper_im(arr: Arc<Vec<i32>>, left: usize, right: usize, merged_arr: Arc<Mutex<Vec<i32>>>, depth: usize)  {
    if right - left > 1 {
        let mid = (left + right) / 2;
        let new_depth = depth + 1;

        let arr_cloned1 = Arc::clone(&arr);
        let arr_cloned2 = Arc::clone(&arr);
        let left_arr = Arc::new(Mutex::new(Vec::with_capacity(mid - left)));
        let right_arr = Arc::new(Mutex::new(Vec::with_capacity(right - mid)));
        let left_arr_cloned = Arc::clone(&left_arr);
        let right_arr_cloned = Arc::clone(&right_arr);
        if new_depth < MAX_THREADS {
    
            let handle1 = thread::spawn(move || {
                merge_mt_helper_im(arr_cloned1, left, mid, left_arr_cloned, new_depth);
            });

            let handle2 = thread::spawn(move || {
                merge_mt_helper_im(arr_cloned2, mid, right, right_arr_cloned, new_depth);
            });
            handle1.join().unwrap();
            handle2.join().unwrap();
        } else {
            merge_mt_helper_im(arr_cloned1, left, mid, left_arr_cloned, new_depth);
            merge_mt_helper_im(arr_cloned2, mid, right, right_arr_cloned, new_depth);
        }
    
        merge_mt_im(left_arr, right_arr , Arc::clone(&merged_arr));
    } else if right - left == 1 {
        merge_mt_im_base(Arc::clone(&arr), left, Arc::clone(&merged_arr));
    }  
} 

fn merge_mt_im(left_arr: Arc<Mutex<Vec<i32>>>, right_arr: Arc<Mutex<Vec<i32>>>, merged_arr: Arc<Mutex<Vec<i32>>>) {
    let mut merged_arr_ref = merged_arr.lock().unwrap();
    let mut left_arr_ref = left_arr.lock().unwrap();
    let mut right_arr_ref = right_arr.lock().unwrap();
    
    let left_len = left_arr_ref.len();
    let right_len = right_arr_ref.len();

    let mut left_ptr = 0;
    let mut right_ptr = 0;

    while left_ptr < left_len && right_ptr < right_len {
        if left_arr_ref[left_ptr] < right_arr_ref[right_ptr] {
            merged_arr_ref.push(left_arr_ref[left_ptr]);
            left_ptr += 1;
        } else {
            merged_arr_ref.push(right_arr_ref[right_ptr]);
            right_ptr += 1;
        }
    }

    while left_ptr < left_len {
        merged_arr_ref.push(left_arr_ref[left_ptr]);
        left_ptr += 1;
    }

    while right_ptr < right_len {
        merged_arr_ref.push(right_arr_ref[right_ptr]);
        right_ptr += 1;
    }
}

fn merge_mt_im_base(arr: Arc<Vec<i32>>, left: usize, merged_arr: Arc<Mutex<Vec<i32>>>) {
    let mut merged_arr_ref = merged_arr.lock().unwrap();
    merged_arr_ref.push(arr[left]);
}
