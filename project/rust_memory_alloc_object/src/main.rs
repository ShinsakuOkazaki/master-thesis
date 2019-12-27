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

    let mut elements = Vec::new();
    let mut rng = thread_rng();
    for _i in 0..10 {
        let total_order = rng.gen::<i32>();
        let weight_order = rng.gen::<f32>();
        let zip_code : String = thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(10)
                                .collect();
        let customer = Customer::create(total_order, weight_order, zip_code);
        elements.push(customer);
    }
    
    let start_add = Instant::now();
    
    for _i in 0..size {
        vector.push(elements.pop());
    }

    let elapsed_add = start_add.elapsed().as_nanos();
    let elapsed_total = start_init.elapsed().as_nanos();

    let output = format!("[CharArray]#{:?}#{:?}#{:?}#{:?}#{:?}\n", initialization, size, elapsed_init, elapsed_add, elapsed_total);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");
}

pub struct Customer {
    total_order: i32,
    weight_order: f32,
    zip_code: String,
}

impl Customer {
    pub fn create(total_order: i32, weight_order: f32, zip_code: String) -> Customer {
        Customer{
            total_order: total_order,
            weight_order: weight_order,
            zip_code: zip_code
        }
    }
}