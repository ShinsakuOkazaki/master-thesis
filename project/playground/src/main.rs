
extern crate rand;
extern crate serde;
extern crate bytes;
use std::env;
use std::time::Instant;

use std::sync::Arc;
mod objects;
//use objects::customer::*;
//use objects::order::*;
use objects::field::*;
//use objects::dropping::*;
//use objects::access::*;
use objects::sort::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let mut nums1 = get_integer_vector(size);
    let n1 = nums1.len();
    println!("Initial: {:?}", nums1);
    mergesort_st(&mut nums1[..], 0, n1);
    println!("Sort with ST: {:?}", nums1);
    let nums2 = get_integer_vector(size);
    let n2 = nums2.len();
    let merged = mergesort_mt(nums2, 0, n2);
    println!("Sort with MT{:?}", merged);
    let nums3 = get_integer_vector(size);
    let n3 = nums3.len();
    let num3_atomic = Arc::new(Mutex::new(nums3));
    mergesort_mt_mutex(Arc::clone(&num3_atomic), 0, n3);
    println!("Sort with Mutex{:?}", num3_atomic);
}