use std::thread;
pub use std::sync::{mpsc, Mutex, Arc};
use std::cell::RefCell;
use std::rc::Rc;
pub use crossbeam::atomic::AtomicCell;
use crate::objects::customer::*;
use std::cmp::Ordering;
use std::marker::{Send, Sync};
use std::mem;
use std::collections::{LinkedList, VecDeque};

const MAX_THREADS: usize = 4;



pub fn mergesort_vecdeque<T: 'static>(mut arr:VecDeque<Arc<T>>, left: usize, right: usize) -> VecDeque<Arc<T>> 
    where T: Clone + Customer + PartialOrd + Send + Sync
{
    let l = arr.len();
    return merge_helper_vecdeque(Arc::new(arr), 0, l, 0);
}


fn merge_helper_vecdeque<T: 'static>(arr: Arc<VecDeque<Arc<T>>>, left: usize, right: usize, depth: usize) -> VecDeque<Arc<T>>  
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
                let sorted = merge_helper_vecdeque(arr_cloned1, left, mid, new_depth);
                sender.send(sorted).unwrap();
            });
            arr_right = merge_helper_vecdeque(arr_cloned2, mid, right, new_depth);
            arr_left = receiver.recv().unwrap();
        } else {
            arr_left = merge_helper_vecdeque(arr_cloned1, left, mid, new_depth);
            arr_right = merge_helper_vecdeque(arr_cloned2, mid, right, new_depth);
        }
        return merge_vecdeque(arr_left, arr_right);
    }
    return merge_vecdeque_base(arr, left);
}

fn merge_vecdeque<T: 'static>(arr_left: VecDeque<Arc<T>>, arr_right: VecDeque<Arc<T>>) -> VecDeque<Arc<T>> 
    where T: Clone + Customer + PartialOrd + Send + Sync
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

fn merge_vecdeque_base<T: 'static>(arr: Arc<Vec<Arc<T>>>, left: usize) -> Vec<Arc<T>> 
    where T: Clone + Customer + PartialOrd + Send + Sync
{
    let mut arr_merged = Vec::with_capacity(1);
    arr_merged.push(Arc::clone(&arr[left]));
    return arr_merged;
}

/////////////////////////////////////
/////////////////////////////////////
/////////////////////////////////////
/////////////////////////////////////

pub fn mergesort_linkedlist<T: 'static>(mut head: LinkedList<T>) ->LinkedList<T> 
    where T: Clone + Customer + PartialOrd + Send + Sync
{
    let l = head.len();
    return merge_helper_linkedlist(head, 0, l, 0);
}

fn merge_helper_linkedlist<T: 'static>(mut head: LinkedList<T>, left: usize, right: usize, depth: usize) ->LinkedList<T> 
    where T: Clone + Customer + PartialOrd + Send + Sync
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
    where T: Clone + Customer + PartialOrd + Send + Sync
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
    where T: Clone + Customer + PartialOrd + Send + Sync
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
    where T: Clone + Customer + PartialOrd + Send + Sync
{
    return merge_mt_helper_mp_generic(Arc::new(arr), left, right, 0);
}

fn merge_mt_helper_mp_generic<T: 'static>(arr: Arc<Vec<T>>, left: usize, right: usize, depth: usize) -> Vec<T> 
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

fn merge_mt_mp_base_generic<T>(arr: Arc<Vec<T>>, left: usize) -> Vec<T> 
    where T: Clone + Customer + PartialOrd + Send + Sync
{
    let mut arr_merged = Vec::with_capacity(1);
    arr_merged.push(arr[left].clone());
    return arr_merged;
}




//////////////////////////////////////
//////////////////////////////////////
/////////////////////////////////////////
///////////////////////////////////////// 
pub fn mergesort_mt_mp_customer(arr:Vec<CustomerOwned>, left: usize, right: usize) -> Vec<CustomerOwned> {
    return merge_mt_helper_mp_customer(arr, left, right, 0);
}

fn merge_mt_helper_mp_customer(arr: Vec<CustomerOwned>, left: usize, right: usize, depth: usize) -> Vec<CustomerOwned> {
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
                let left_sorted = merge_mt_helper_mp_customer(left_ptr.into_inner(), 0, arr_left_len, new_depth);
                sender.send(left_sorted).unwrap();
            });
            arr_right = merge_mt_helper_mp_customer(arr_right, 0, arr_right_len, new_depth);
            arr_left = receiver.recv().unwrap();
        } else {
            arr_left = merge_mt_helper_mp_customer(arr_left, 0, arr_left_len, new_depth);
            arr_right = merge_mt_helper_mp_customer(arr_right, 0, arr_right_len, new_depth);
        }
        return merge_mt_mp_customer(arr_left, arr_right);
    }
    return arr;
}


fn merge_mt_mp_customer(mut arr_left: Vec<CustomerOwned>, mut arr_right: Vec<CustomerOwned>) -> Vec<CustomerOwned> {
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

