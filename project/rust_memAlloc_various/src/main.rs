extern crate rand;
use std::env;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::ptr;
use std::mem;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use rand::distributions::{Distribution, Uniform};

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let size: usize = args[1].parse().unwrap();
    let method: i32 = args[2].parse().unwrap();
    let eltype: i32 = args[3].parse().unwrap();
    let mutability: bool = args[4].parse().unwrap();
    add_element(size, method, eltype, mutability);
}

fn add_element(size:usize, method: i32, eltype: i32, mutability: bool) {
    match mutability {
        true => mutable_access(size, method, eltype),
        false => immutable_access(size, method, eltype),
        _ => println!("Wrong type")
    }
}

fn mutable_access(size:usize, method: i32, eltype: i32) {
    match eltype {
        1 => add_integer_mutable_access(size, method, eltype, true),
        2 => add_string_mutable_access(size, method, eltype, true),
        _ => println!("Wrong type")
    }
}

fn immutable_access(size:usize, method: i32, eltype: i32) {
    match eltype {
        1 => add_integer_immutable_access(size, method, eltype, false),
        2 => add_string_immutable_access(size, method, eltype, false),
        _ => println!("Wrong type")
    }
}

fn add_string_immutable_access(size: usize, method: i32, eltype: i32, mutability: bool) {
    println!("size: {}", size);

    let start_init = Instant::now();
    // We need to make sure that vector has enough size to copy source vector.
    let mut distination = Vec::with_capacity(size);
    let elapsed_init = start_init.elapsed().as_nanos();

    let mut source = get_vector_string(size);

    // Make condition when clone is used and otherwise.
    let elapsed_add = do_clone_from(&mut distination, &mut source);

    let immutable = distination;

    let elapsed_access = access_string_test_immutable(&immutable, &source);
    
    let elapsed_total = start_init.elapsed().as_nanos();

    write_to_file(method, eltype, mutability, size , elapsed_init, elapsed_add, elapsed_access, elapsed_total);
}

fn add_string_mutable_access(size: usize, method: i32, eltype: i32, mutability: bool) {
    println!("size: {}", size);

    let start_init = Instant::now();
    // We need to make sure that vector has enough size to copy source vector.
    let mut distination = Vec::with_capacity(size);
    let elapsed_init = start_init.elapsed().as_nanos();

    let mut source = get_vector_string(size);

    // Make condition when clone is used and otherwise.
    let elapsed_add = do_clone_from(&mut distination, &mut source);

    let elapsed_access = access_string_test_mutable(&mut distination, &source);
    
    let elapsed_total = start_init.elapsed().as_nanos();

    write_to_file(method, eltype, mutability, size , elapsed_init, elapsed_add, elapsed_access, elapsed_total);
}

fn add_integer_immutable_access(size: usize, method: i32, eltype: i32, mutability: bool) {
    println!("size: {}", size);

    let start_init = Instant::now();
    // We need to make sure that vector has enough size to copy source vector.
    let mut distination = Vec::with_capacity(size);
    let elapsed_init = start_init.elapsed().as_nanos();

    let mut source = get_vector_integer(size);
    
    // Make condition when clone is used and otherwise.
    let elapsed_add = unsafe {
        select_experiment(method, &mut distination, &mut source, size)
    };

    let immutable = distination;

    let elapsed_access = access_integer_test_immutable(&immutable, &source);
    
    let elapsed_total = start_init.elapsed().as_nanos();

    write_to_file(method, eltype, mutability, size , elapsed_init, elapsed_add, elapsed_access, elapsed_total);
}

fn add_integer_mutable_access(size: usize, method: i32, eltype: i32, mutability: bool) {
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
    let elapsed_add = unsafe {
        select_experiment(method, &mut distination, &mut source, size)
    };

    let elapsed_access = access_integer_test_mutable(&mut distination, &source);
    
    let elapsed_total = start_init.elapsed().as_nanos();

    write_to_file(method, eltype, mutability, size , elapsed_init, elapsed_add, elapsed_access, elapsed_total);
}

fn get_vector_string(size: usize) -> Vec<String>{
    let dist = Uniform::from(3..7);
    let mut source = Vec::with_capacity(size);
    for _i in 0..size {
        let rand_string: String = thread_rng()
                            .sample_iter(&Alphanumeric)
                            .take(dist.sample(&mut thread_rng()))
                            .collect();
        source.push(rand_string);    
    }
    return source;
}

fn get_vector_integer(size: usize) -> Vec<i32> {
    let mut source = Vec::with_capacity(size);
    for i in 0..size {
        source.push(i as i32);
    }
    return source;
}

fn write_to_file(method: i32, eltype: i32, mutability: bool, size: usize, elapsed_init: u128, elapsed_add: u128, elapsed_access: u128,  elapsed_total: u128) {
    let output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}#{:?}#{:?}#{:?}#{:?}\n", 
                        get_methodName(method), get_elementType(eltype), get_mutability(mutability), 
                        size, elapsed_init, elapsed_add, elapsed_access, elapsed_total);
    println!("{}",output);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");
}

fn access_integer_test_mutable(distination :&mut Vec<i32>, source: &Vec<i32>) -> u128 {
    let start_access = Instant::now();
    let len = distination.len();
    let mut sum: i64 = 0;
    for i in 0..len {
        sum += (distination[i] as i64);
    }
    let elapsed_access = start_access.elapsed().as_nanos();

    let mut true_sum: i64 = 0;
    for i in 0..len {
        true_sum += (i as i64);
    }
    let correct = (sum == true_sum);
    println!("{}", correct);
    return elapsed_access;
}

fn access_integer_test_immutable(distination :&Vec<i32>, source: &Vec<i32>) -> u128 {
    let start_access = Instant::now();
    let len = distination.len();
    let mut sum: i64 = 0;
    for i in 0..len {
        sum += (distination[i] as i64);
    }
    let elapsed_access = start_access.elapsed().as_nanos();

    let mut true_sum: i64 = 0;
    for i in 0..len {
        true_sum += (i as i64);
    }
    let correct = (sum == true_sum);
    println!("{}", correct);
    return elapsed_access;
}

fn access_string_test_mutable(distination :&mut Vec<String>, source: &Vec<String>) -> u128 {
    let start_access = Instant::now();
    let len = distination.len();
    let mut sum: i64 = 0;
    for i in 0..len {
        sum += (distination[i].len() as i64);
    }
    let elapsed_access = start_access.elapsed().as_nanos();
    let mut true_sum: i64 = 0;
    for i in 0..len {
        true_sum += (source[i].len() as i64);
    }

    let correct = (sum == true_sum);
    println!("{}", correct);
    return elapsed_access;
}

fn access_string_test_immutable(distination :&Vec<String>, source: &Vec<String>) -> u128 {
    let start_access = Instant::now();
    let len = distination.len();
    let mut sum: i64 = 0;
    for i in 0..len {
        sum += (distination[i].len() as i64);
    }
    let elapsed_access = start_access.elapsed().as_nanos();
    let mut true_sum: i64 = 0;
    for i in 0..len {
        true_sum += (source[i].len() as i64);
    }

    let correct = (sum == true_sum);
    println!("{}", correct);
    return elapsed_access;
}

unsafe fn select_experiment<T: Eq + Ord + Clone>(method: i32, dst: &mut Vec<T>, src: &mut Vec<T>, size: usize) -> u128 {
    match method {
        1 => memory_copy(dst, src, size),
        2 => one_by_one(dst, src, size),
        3 => do_clone(dst, src),
        4 => do_clone_from(dst, src),
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

fn do_clone<T: Eq + Ord + Clone>(dst: &mut Vec<T>, src: &mut Vec<T>) -> u128 {
    let start_add = Instant::now();
    *dst = (*src).clone();
    start_add.elapsed().as_nanos() 
}

fn do_clone_from<T: Eq + Ord + Clone>(dst: &mut Vec<T>, src: &mut Vec<T>) -> u128 {
    let start_add = Instant::now();
    (*dst).clone_from(src);
    start_add.elapsed().as_nanos()
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

fn get_mutability(mutability: bool) -> String {
    match mutability {
        true => String::from("mutable"),
        false => String::from("immutable"),
        _ => String::from(""),
    }
}