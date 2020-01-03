extern crate rand;

use std::env;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::prelude::*;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    
    let v = vector_addition(2, true, 2);
    println!("{:?}", v);
}

fn vector_addition(size: usize, initialization: bool, elementtype: i32) {
    match size {
        1 => add_to_vector(10, initialization, elementtype), 
        2 => add_to_vector(100, initialization, elementtype), 
        3 => add_to_vector(1000, initialization, elementtype), 
        4 => add_to_vector(10000, initialization, elementtype), 
        _ => println!("Invalid input!"),
    }
}

fn vector_initialization(size: usize, initialization: bool, ) -> std::vec::Vec<i32> {
    let mut vector;
    if initialization {
        vector = Vec::with_capacity(size);
    } else {
        vector = Vec::new();
    }
    return vector;
}


fn get_elementsVec(size: usize, elementtype: i32) {
    let mut elements = Vec::new();
    match elementtype {
        1 => element_integer(size),
        2 => element_string(size),
        3 => elements_charArr(size),
        4 => elements_object(size),
        _ => println!("Invalid input!"),
    }
}


fn element_integer(size: usize) -> std::vec::Vec<i32> {
    let mut elements = Vec::new();
    for e in 0..size {
        elements.push(e as i32);
    }
    return elements;
}

fn element_string(size: usize) -> std::vec::Vec<String> {
    let mut elements = Vec::new();
    for _i in 0..size {
        let rand_string: String = thread_rng()
                            .sample_iter(&Alphanumeric)
                            .take(10)
                            .collect();
        elements.push(rand_string);
    }
    return elements;
}

fn elements_charArr(size: usize) -> std::vec::Vec<[char;10]> {
    let mut elements = Vec::new();
    for _i in 0..size {
        let char_arr = ['a', 'b', 'c', 'd', 'e',
                        'f', 'g', 'h', 'i', 'j'];
        elements.push(char_arr);
    }
    return elements;
}

fn elements_object(size: usize) -> std::vec::Vec<Customer> {
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
    return elements;
}

fn add_to_vector(size: usize, initialization: bool, elementtype: i32) {
    
    // Initialize vector.
    println!("size: {}, initialization: {}", size, initialization);
    let start_init = Instant::now();
    let mut vector = vector_initialization(size, initialization);
    let elapsed_init = start_init.elapsed().as_nanos();

    let mut elements;

    if elementtype == 1 {
        elements = get_elementsVec(size: usize, elementtype: i32)
    }
    
    let start_add = Instant::now();
    
    for _i in 0..size {
        vector.push(elements.pop());
    }

    let elapsed_add = start_add.elapsed().as_nanos();
    let elapsed_total = start_init.elapsed().as_nanos();

    let output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}#{:?}#{:?}\n", elementtype, initialization, size, elapsed_init, elapsed_add, elapsed_total);

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
        Customer {
            total_order: total_order,
            weight_order: weight_order,
            zip_code: zip_code
        }
    }
}