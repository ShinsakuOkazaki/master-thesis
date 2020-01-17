extern crate rand;

use std::env;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::ptr;

use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {

    let args: Vec<String> = env::args().collect();
    
    let size: usize = args[1].parse().unwrap();
    add_to_imutable(size);

}

fn add_to_imutable(size: usize) {
    
    println!("size: {}", size);
    let start_init = Instant::now();
    let vector;
    vector = Vec::with_capacity(size);
    

    let elapsed_init = start_init.elapsed().as_nanos();

    let mut src_arr = Vec::with_capacity(size);
    let mut rng = thread_rng();
    for _i in 0..10 {
        let total_order = rng.gen::<i32>();
        let weight_order = rng.gen::<f32>();
        let zip_code : String = thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(10)
                                .collect();
        let customer = Customer::create(total_order, weight_order, zip_code);
        src_arr.push(customer);
    }
    
    let mut dst = vector;
    let src = &src_arr;
    let dst_ptr = dst.as_mut_ptr();
    let src_ptr = src.as_ptr();
    let start_add = Instant::now();
    unsafe {
        ptr::copy_nonoverlapping(src_ptr, dst_ptr, size);
    }

    let elapsed_add = start_add.elapsed().as_nanos();
    let elapsed_total = start_init.elapsed().as_nanos();

    let output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}\n", size, elapsed_init, elapsed_add, elapsed_total);

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