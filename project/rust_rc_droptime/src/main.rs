
extern crate rand;
extern crate serde;
extern crate bytes;
extern crate tpch_generator;

use tpch_generator::objects::access;
use tpch_generator::objects::customer;
use tpch_generator::objects::dropping; 
use std::env;
use std::time::Instant;
use std::io::prelude::*;
use std::fs::OpenOptions;

// mod objects;
// use objects::customer::*;
// use objects::order::*;
// use objects::field::*;
// use objects::dropping::*;
// use objects::access::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let lineitem_file: &str = args.get(1).unwrap();
    let order_file: &str = args.get(2).unwrap();
    let customer_file: &str = args.get(3).unwrap();
    let lineitem_size: usize = args[4].parse::<usize>().unwrap();
    let order_size: usize = args[5].parse::<usize>().unwrap(); 
    let customer_size: usize = args[6].parse::<usize>().unwrap();
    let field: i32 = args[7].parse::<i32>().unwrap();
    run(&lineitem_file, &order_file, &customer_file, lineitem_size, order_size, customer_size, field);
}

fn run(lineitem_file: &str, order_file: &str, customer_file: &str, lineitem_size: usize, order_size: usize, customer_size: usize, field: i32) {
    match field {
        1 => run_ex_owned(&lineitem_file, &order_file, &customer_file, lineitem_size, order_size, customer_size),
        2 => run_ex_borrowed(&lineitem_file, &order_file, &customer_file, lineitem_size, order_size, customer_size),
        3 => run_ex_rc(&lineitem_file, &order_file, &customer_file, lineitem_size, order_size, customer_size),
        _ => println!("Wrong input!")
    }
}


fn run_ex_owned(lineitem_file: &str, order_file: &str, customer_file: &str, lineitem_size: usize, order_size: usize, customer_size: usize ) {
    let start = Instant::now();
    // Create vector of CustomerOwned objects and take creation time.
    let (elapsed_create, mut customers) = customer::create_objects_owned(&lineitem_file, &order_file, &customer_file, lineitem_size, order_size, customer_size);
    let size = customers.len();
    let elapsed_access = access::access_owned(&customers);
    // Access to every feild of each object in the vector and take access time.
    //let elapsed_access = access_owned(& customers);
    let elapsed_drop = dropping::drop_owned(&mut customers);
    let elapsed_total = start.elapsed().as_micros();
    write_to_file(size, "own", elapsed_create, elapsed_access, elapsed_drop, elapsed_total);
}


fn run_ex_borrowed(lineitem_file: &str, order_file: &str, customer_file: &str, lineitem_size: usize, order_size: usize, customer_size: usize) {
    let start = Instant::now();
    let (_, customers_owned) = customer::create_objects_owned(&lineitem_file, &order_file, &customer_file, lineitem_size, order_size, customer_size);
    // Create vector of CustomerBorrowed objects and take creation time.
    let (elapsed_create, mut customers_borrowed) = customer::create_customer_borrowed_vector(&customers_owned);
    let size = customers_borrowed.len();
    // Access to every feild of each object in the vector and take access time.
    let elapsed_access = access::access_borrowed(&customers_borrowed);
    let elapsed_drop = dropping::drop_borrowed(&mut customers_borrowed);
    let elapsed_total = start.elapsed().as_micros();
    write_to_file(size, "reference", elapsed_create, elapsed_access, elapsed_drop, elapsed_total);
}

// Function to run experiment for object whose fields are slice.
fn run_ex_rc(lineitem_file: &str, order_file: &str, customer_file: &str, lineitem_size: usize, order_size: usize, customer_size: usize) {
    let start = Instant::now();
    // Create vector of CustomerSlice objects and take creation time.
    let (elapsed_create, mut customers) = customer::create_objects_rc(&lineitem_file, &order_file, &customer_file, lineitem_size, order_size, customer_size);
        // Access to every feild of each object in the vector and take access time. 
    let size = customers.len();
    let elapsed_access = access::access_rc(&customers);
    let elapsed_drop = dropping::drop_rc(&mut customers);
    let elapsed_total = start.elapsed().as_micros();
    write_to_file(size, "rc", elapsed_create, elapsed_access, elapsed_drop, elapsed_total);
}

// Function to write result to file.
fn write_to_file(size: usize, field: &str, elapsed_create: u128, elapsed_access: u128, elapsed_drop: u128, elapsed_total: u128) {
    let output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}#{:?}#{:?}\n", 
                         size, field, elapsed_create, elapsed_access, elapsed_drop, elapsed_total);
    println!("{}",output);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");
}

