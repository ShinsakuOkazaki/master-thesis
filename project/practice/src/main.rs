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

pub fn mergesort_vecdeque(arr: Vec<i32>, left: usize, right: usize) -> Vec<i32>{
    let mut res = Vec::with_capacity(right);
    merge_helper_vecdeque(&mut arr[..], left, right, 0, &mut res[..]);
    return res
}


fn merge_helper_vecdeque(arr: &mut [i32], left: usize, right: usize, depth: usize, res: &mut [i32]) {
    if right - left > 1 {
        let mid = (left + right) / 2;
        let new_depth = depth + 1;
        let n = arr.len();
        let res_left = Vec::with_capacity(n);
        let res_right = Vec::with_capacity(n);
        let (arr_right, arr_left) = arr.split_at_mut(mid);
        if new_depth < MAX_THREADS {
            crossbeam::scope(|scope| {
                let arr_left_ptr = &mut arr_left[..];
                let arr_right_ptr = &mut arr_right[..];
                let ref_left = &mut res_left[..];
                let ref_right = &mut res_right[..];
                scope.spawn(move |_| {
                    merge_helper_vecdeque(arr_left_ptr, left, mid, new_depth, ref_left);
                });
                scope.spawn(move |_| {
                    merge_helper_vecdeque(arr_right_ptr, mid, right, new_depth, ref_right);
                })
            });
        } else {
            merge_helper_vecdeque(&mut arr_left[..], left, mid, new_depth, &mut res_left[..]);
            merge_helper_vecdeque(&mut arr_right[..], mid, right, new_depth, &mut res_right[..]);
        }
        merge_vecdeque(&mut res_left[..], &mut res_right[..], &mut res[..]);
    } else if right - left == 1 {
        merge_vecdeque_base(&mut arr[..], left, &mut res[..]);
    } 
}

fn merge_vecdeque(arr_left: &mut [i32], arr_right: &mut [i32], res: &mut [i32]) {
    let arr_left_len = arr_left.len();
    let arr_right_len = arr_right.len();
    
    let left_ptr = 0;
    let right_ptr = 0;
    let current = 0;

    while left_ptr < arr_left_len && right_ptr < arr_right_len {
        if (arr_left[left_ptr] < arr_right[right_ptr]) {
            res[current] = mem::replace(&mut arr_left[left_ptr], i32::default());
            left_ptr += 1;
            current += 1;
        } else {
            res[current] = mem::replace(&mut arr_right[right_ptr], i32::default());
            right_ptr += 1;
            current += 1;
        }
    }

    if left_ptr < arr_left_len {
        res[current] =  mem::replace(&mut arr_left[left_ptr], i32::default()); 
        left_ptr += 1;
        current += 1;
    }

    if right_ptr < arr_right_len {
        res[current] = mem::replace(&mut arr_right[right_ptr], i32::default());  
    }
}

fn merge_vecdeque_base(arr: &mut [i32], left: usize, res: &mut [i32]) {
    res[left] = mem::replace(&mut arr[0], i32::default())
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