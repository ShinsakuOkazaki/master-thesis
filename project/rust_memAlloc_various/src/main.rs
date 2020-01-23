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
    let eltype: i32 = args[3].parse().unwrap();
    add_element(size, method, eltype);
}

fn add_element(size:usize, method: i32, eltype: i32) {
    match eltype {
        1 => add_integer(size, method, eltype),
        2 => add_string(size, method, eltype),
        _ => println!("Wrong type")
    }
}

fn add_string(size: usize, method: i32, eltype: i32) {
    println!("size: {}", size);

    let start_init = Instant::now();
    // We need to make sure that vector has enough size to copy source vector.
    let mut distination = Vec::with_capacity(size);
    let elapsed_init = start_init.elapsed().as_nanos();

    let mut source = Vec::with_capacity(size);
    for _i in 0..size {
        let rand_string: String = thread_rng()
                            .sample_iter(&Alphanumeric)
                            .take(10)
                            .collect();
        source.push(rand_string);    
    }

    // Make condition when clone is used and otherwise.
    let elapsed_add = unsafe {if method == 3 {
        let start_add = Instant::now();
        distination = source.clone();
        start_add.elapsed().as_nanos()
    }else if method == 4 {
        let start_add = Instant::now();
        distination.clone_from(&source);
        start_add.elapsed().as_nanos() 
    } else {
        select_experiment(method, &mut distination, &mut source, size)
    }};

    
    
    let elapsed_total = start_init.elapsed().as_nanos();

    println!("{:?}", distination[5]);

    write_to_file(method, eltype, size, elapsed_init, elapsed_add, elapsed_total);
}

fn add_integer(size: usize, method: i32, eltype: i32) {
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
    } else if method == 4 {
        let start_add = Instant::now();
        distination.clone_from(&source);
        start_add.elapsed().as_nanos() 
    } else {
        select_experiment(method, &mut distination, &mut source, size)
    }};

    
    
    let elapsed_total = start_init.elapsed().as_nanos();

    println!("{:?}", distination[5]);

    write_to_file(method, eltype, size, elapsed_init, elapsed_add, elapsed_total);
}

fn write_to_file(method: i32, eltype: i32, size: usize, elapsed_init: u128, elapsed_add: u128, elapsed_total: u128) {
    let output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}#{:?}#{:?}\n", get_methodName(method), get_elementType(eltype), size, elapsed_init, elapsed_add, elapsed_total);
    println!("{}",output);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");
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

fn get_methodName(method: i32) -> String {
    match method {
        1 => String::from("memcpy"),
        2 => String::from("onebyone"),
        3 => String::from("clone"),
        4 => String::from("clone_from"),
        _ => String::from(""), 
    }
}

fn get_elementType(eltype: i32) -> String {
    match eltype {
        1 => String::from("Integer"),
        2 => String::from("String"),
        _ => String::from(""),
    }
}