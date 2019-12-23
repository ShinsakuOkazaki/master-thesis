extern crate rand;

use std::env;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::prelude::*;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let size: usize = args[1].parse().unwrap();
    let initialization: bool = args[2].parse().unwrap();
    vector_addition(size, initialization)

}
fn vector_addition(size: usize, initialization: bool) {
    match size {
        1 => add_to_vector(10, initialization), 
        2 => add_to_vector(100, initialization), 
        3 => add_to_vector(1000, initialization), 
        4 => add_to_vector(10000, initialization), 
        _ => println!("Invalid input!"),
    }
}

fn add_to_vector(size: usize, initialization: bool) {
    
    println!("size: {}, initialization: {}", size, initialization);
    let start_init = Instant::now();
    let mut vector;
    if initialization {
        vector = Vec::with_capacity(size);
    } else {
        vector = Vec::new();
    }

    let elapsed_init = start_init.elapsed().as_nanos();

    let mut strings_arr = Vec::with_capacity(size);
    for _i in 0..size {
        let rand_string: String = thread_rng()
                            .sample_iter(&Alphanumeric)
                            .take(10)
                            .collect();
        strings_arr.push(rand_string);
        
    }
    
    let start_add = Instant::now();
    
    for _i in 0..size {
        vector.push(strings_arr.pop());
    }

    let elapsed_add = start_add.elapsed().as_nanos();
    let elapsed_total = start_init.elapsed().as_nanos();

    let output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}#{:?}\n", initialization, size, elapsed_init, elapsed_add, elapsed_total);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");
}
