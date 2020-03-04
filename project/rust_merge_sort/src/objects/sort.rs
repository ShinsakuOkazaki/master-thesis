use std::thread;
pub use std::sync::{mpsc, Arc};
use std::cell::RefCell;
use crate::objects::customer::*;
//use std::cmp::Ordering;
use std::marker::{Send, Sync};
use std::collections::{LinkedList, VecDeque};
use std::default::Default;
use std::time::Instant;

const MAX_THREADS: usize = 4;

pub fn mergesort_vecdeque<T: 'static>(arr:VecDeque<T>, left: usize, right: usize) -> (u128, VecDeque<T>) 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    let start = Instant::now();
    let res = merge_helper_vecdeque(Arc::new(arr), left, right, 0);
    let elapsed = start.elapsed().as_micros(); 
    return (elapsed, res)
}


fn merge_helper_vecdeque<T: 'static>(arr: Arc<VecDeque<T>>, left: usize, right: usize, depth: usize) -> VecDeque<T>
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    if right - left > 1 {
        let mid = (left + right) / 2;
        let new_depth = depth + 1;
        let arr_cloned1 = Arc::clone(&arr);
        let arr_cloned2 = Arc::clone(&arr);
        let arr_right;
        let arr_left;

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

fn merge_vecdeque<T: 'static>(mut arr_left: VecDeque<T>, mut arr_right: VecDeque<T>) -> VecDeque<T> 
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

fn merge_vecdeque_base<T: 'static>(arr: Arc<VecDeque<T>>, left: usize) -> VecDeque<T>
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    let mut arr_merged = VecDeque::with_capacity(1);
    arr_merged.push_back(arr[left].clone());
    return arr_merged;
}


//////////////////////
/////////////////////
////////////////////////

pub fn mergesort_vecdeque_slice<T: 'static>(mut arr: Vec<T>, len: usize) -> (u128, VecDeque<T>) 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    let start = Instant::now();
    let res = merge_helper_vecdeque_slice(&mut arr[..], len, 0);
    let elapsed = start.elapsed().as_micros(); 
    return (elapsed, res);
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
    //let temp = mem::replace(&mut arr[0], T::default());
    arr_merged.push_back(arr[0].clone());
    return arr_merged;
}


///////////////////////////
//////////////////////////
//////////////////////////////



pub fn mergesort_linkedlist<T: 'static>(head: LinkedList<T>) -> (u128, LinkedList<T>)
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    let start = Instant::now();
    let l = head.len();
    let res = merge_helper_linkedlist(head, 0, l, 0);
    let elapsed = start.elapsed().as_micros();
    return (elapsed, res);
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
            let (sender1, receiver1) = crossbeam::channel::unbounded();
            let (sender2, receiver2) = crossbeam::channel::unbounded(); 
            let left_ptr = RefCell::new(left_head);
            let right_ptr = RefCell::new(right_head);
            let _ = thread::spawn(move || {
                let sorted = merge_helper_linkedlist(left_ptr.into_inner(), 0, left_len, new_depth);
                sender1.send(sorted).unwrap();
            });
            let _ = thread::spawn(move || {
                let sorted = merge_helper_linkedlist(right_ptr.into_inner(), 0, right_len, new_depth);
                sender2.send(sorted).unwrap();
            });
            left_head = receiver1.recv().unwrap();
            right_head = receiver2.recv().unwrap(); 
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