extern crate rand;
use std::env;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::ptr;
use std::mem;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let size: usize = args[1].parse().unwrap();
    let method: i32 = args[2].parse().unwrap();
    add_elements(size, method);

}

fn add_elements(size: usize, method: i32) {
    println!("size: {}", size);

    let start_init = Instant::now();
    // We need to make sure that vector has enough size to copy source vector.
    let mut distination = Vec::with_capacity(size);
    let elapsed_init = start_init.elapsed().as_nanos();

    let mut source = Vec::with_capacity(size);
    for i in 0..size {
        source.push(i as i32);
    }
    
    // Make condition when clone is used and otherwise.
    let elapsed_add = unsafe {if method == 3 {
        let start_add = Instant::now();
        distination = source.clone();
        start_add.elapsed().as_nanos()
    } else {
        select_experiment(method, &mut distination, &mut source, size)
    }};

    
    
    let elapsed_total = start_init.elapsed().as_nanos();

    println!("{:?}", distination[5]);

    let output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}\n", size, elapsed_init, elapsed_add, elapsed_total);

    println!("{}",output);
}

unsafe fn select_experiment<T>(method: i32, dst: &mut Vec<T>, src: &mut Vec<T>, size: usize) -> u128 {
    match method {
        1 => memory_copy(dst, src, size),
        2 => one_by_one(dst, src, size),
        _ => 0,
    }
}


unsafe fn memory_copy<T>(dst: &mut Vec<T>, src: &mut Vec<T>, size: usize) -> u128 {
    let start_add = Instant::now();
    // Get mutable pointer to distination.
    let dst_ptr = dst.as_mut_ptr();
    // Get immutable pointer to source.
    let src_ptr = src.as_ptr();
    // Set source length to 0.
    src.set_len(0);
    // Copy source to distination.
    ptr::copy(src_ptr, dst_ptr, size);
    // Set size of distionation.
    dst.set_len(size);

    let elapsed_add = start_add.elapsed().as_nanos();
    return elapsed_add;
}

fn one_by_one<T>(dst: &mut Vec<T>, src: &mut Vec<T>, size: usize) -> u128 {

    let start_add = Instant::now();
    // Add elements of source to distination one by one.
    for _i in 0..size {
        dst.push(src.pop().unwrap());
    }

    let elapsed_add = start_add.elapsed().as_nanos();
    return elapsed_add;
}

