use std::thread;
use std::mem;
use std::sync::{mpsc, Arc};
use std::collections::{LinkedList, VecDeque};
use std::cell::RefCell;
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use std::env;
use rand::distributions::{Uniform, Distribution};
use crossbeam;
const MAX_THREADS: usize = 4;
fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let mut nums = get_integer_vector(size);
    println!("Initial: {:?}", nums);
    let res = mergesort_vecdeque(nums, 0, size);
    println!("Sorted: {:?}", res);
}

pub fn mergesort_vecdeque(mut arr: Vec<i32>, left: usize, right: usize) -> VecDeque<i32> {
    let res = merge_helper_vecdeque(&mut arr[..], left, right, 0);
    return res;
}


fn merge_helper_vecdeque(arr: &mut [i32], left: usize, right: usize, depth: usize) -> VecDeque<i32> {
    if right - left > 1 {
        let mid = (left + right) / 2;
        let new_depth = depth + 1;
        let (arr_right, arr_left) = arr.split_at_mut(mid);
        let res_left: VecDeque<i32>;
        let res_right: VecDeque<i32>;
        if new_depth < MAX_THREADS {
            let (left_thread, right_thread) = crossbeam::scope(|scope| {
                
                let left_handler = scope.spawn(move |_| {
                    merge_helper_vecdeque(arr_left, left, mid, new_depth)
                });
                let right_handler = scope.spawn(move |_| {
                    merge_helper_vecdeque(arr_right, mid, right, new_depth)
                });
                
                let l = left_handler.join().unwrap();
                let r = right_handler.join().unwrap();
                (l, r)
            }).unwrap();
            res_left = left_thread;
            res_right = right_thread;
        } else {
            res_left = merge_helper_vecdeque(arr_right, left, mid, new_depth);
            res_right = merge_helper_vecdeque(arr_left, mid, right, new_depth);
        }
        return merge_vecdeque(res_left, res_right);
    }
    return merge_vecdeque_base(arr, left);
}

fn merge_vecdeque(mut arr_left: VecDeque<i32>, mut arr_right: VecDeque<i32>) -> VecDeque<i32> {
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

fn merge_vecdeque_base(arr: &mut [i32], left: usize) -> VecDeque<i32> {
    println!("In base case {:?}", arr);
    let mut arr_merged = VecDeque::with_capacity(1);
    let temp = mem::replace(&mut arr[0], i32::default());
    arr_merged.push_back(temp);
    return arr_merged;
}
///////////////////////
////////////////////////////
/////////////////////////////
pub fn mergesort_linkedlist(mut head: LinkedList<i32>) ->LinkedList<i32> {
    let l = head.len();
    return merge_helper_linkedlist(head, 0, l, 0);
}

fn merge_helper_linkedlist(mut head: LinkedList<i32>, left: usize, right: usize, depth: usize) ->LinkedList<i32> {
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

fn merge_linkedlist(mut left_head: LinkedList<i32>, mut right_head: LinkedList<i32>) ->LinkedList<i32> {
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

fn get_integer_vector(size: usize) -> Vec<i32> {
    let mut integers = Vec::with_capacity(size);
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    let dist = Uniform::from(1..5);
    let mut num: i32;
    for _ in 0..size {
        num = dist.sample(&mut rng) as i32;
        integers.push(num);
    }
    integers    
}