
extern crate rand;
extern crate serde;
extern crate bytes;
use std::env;
use std::time::Instant;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::rc::Rc;
mod objects;
use objects::customer::*;
use objects::order::*;
use objects::field::*;
use objects::dropping::*;
use objects::access::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let field: i32 = args[2].parse().unwrap();
    run(field, size);
}

fn run(field: i32, size: usize) {
    match field {
        1 => run_ex_owned(size),
        2 => run_ex_borrowed(size),
        3 => run_ex_rc(size),
        _ => println!("Wrong input!")
    }
}


fn run_ex_owned(size: usize) {
    let start = Instant::now();
    let order_ids = get_integer_vector(size);
    let num_itemss = get_integer_vector(size);
    let payments = get_float_vector(size);
    let order_times = get_float_vector(size);
    let titles = get_string_vector(size);
    let comments_order = get_string_vector(size);
    let orders = get_order_owned_vector(size, order_ids, num_itemss, payments, order_times, titles, comments_order);
    
    let keys = get_integer_vector(size);
    let ages = get_integer_vector(size);
    let num_purchases = get_integer_vector(size);
    let total_purchases = get_float_vector(size);
    let duration_spents = get_float_vector(size);
    let duration_sinces = get_float_vector(size);
    let zip_codes = get_string_vector(size);
    let addresss = get_string_vector(size);
    let countrys = get_string_vector(size);
    let states = get_string_vector(size);
    let first_names = get_string_vector(size);
    let last_names = get_string_vector(size);
    let provinces = get_string_vector(size);
    let comments = get_string_vector(size);

    // Create vector of CustomerOwned objects and take creation time.
    let (elapsed_create, mut customers) = create_customer_onwed_vector(size, keys, ages, num_purchases, total_purchases, duration_spents, duration_sinces, 
                                                                    zip_codes, addresss, countrys, states, first_names, last_names, provinces, comments, orders);
    // Access to every feild of each object in the vector and take access time.
    //let elapsed_access = access_owned(& customers);
    let elapsed_access = access_owned(& customers);
    let elapsed_drop = drop_owned(&mut customers);
    let elapsed_total = start.elapsed().as_micros();
    write_to_file(size, "own", elapsed_create, elapsed_access, elapsed_drop, elapsed_total);
}


fn run_ex_borrowed(size: usize) {
    let start = Instant::now();
    let order_ids = get_integer_vector(size);
    //let num_itemss = get_integer_vector(size);
    let payments = get_float_vector(size);
    //let order_times = get_float_vector(size);
    let titles = get_string_vector(size);
    //let comments_order = get_string_vector(size);
    let orders = get_order_borrowed_vector(size, &order_ids, &order_ids, &payments, &payments, &titles, &titles);
    
    let keys = get_integer_vector(size);
    //let ages = get_integer_vector(size);
    //let num_purchases = get_integer_vector(size);
    let total_purchases = get_float_vector(size);
    //let duration_spents = get_float_vector(size);
    //let duration_sinces = get_float_vector(size);
    let zip_codes = get_string_vector(size);
    //let addresss = get_string_vector(size);
    //let countrys = get_string_vector(size);
    //let states = get_string_vector(size);
    //let first_names = get_string_vector(size);
    //let last_names = get_string_vector(size);
    //let provinces = get_string_vector(size);
    //let comments = get_string_vector(size);
    
    // Create vector of CustomerBorrowed objects and take creation time.
    let (elapsed_create, mut customers) = create_customer_borrowed_vector(size, &keys, &keys, &keys, &total_purchases, &total_purchases, &total_purchases, 
                                                                &zip_codes, &zip_codes, &zip_codes, &zip_codes, &zip_codes, &zip_codes, &zip_codes, &zip_codes, &orders);
    // Access to every feild of each object in the vector and take access time.
    let elapsed_access = access_borrowed(& customers);
    let elapsed_drop = drop_borrowed(&mut customers);
    let elapsed_total = start.elapsed().as_micros();
    write_to_file(size, "reference", elapsed_create, elapsed_access, elapsed_drop, elapsed_total);
}

// Function to run experiment for object whose fields are slice.
fn run_ex_rc(size: usize) {
    let start = Instant::now();
    // Create String vectors.
    let order_ids = get_integer_rc_vector(size);
    let mut num_itemss: Vec<Rc<i32>> = Vec::with_capacity(size);
    let payments = get_float_rc_vector(size);
    let mut order_times: Vec<Rc<f64>> = Vec::with_capacity(size);
    let titles = get_stirng_rc_vector(size);
    let mut comments_order: Vec<Rc<String>> = Vec::with_capacity(size);
    for i in 0.. size {
        num_itemss.push(Rc::clone(&order_ids[i]));
        order_times.push(Rc::clone(&payments[i]));
        comments_order.push(Rc::clone(&titles[i]));
    }
    let orders = get_order_rc_vector(size, &order_ids, &num_itemss, &payments, &order_times, &titles, &comments_order);
    
    let keys = get_integer_rc_vector(size);
    let mut ages = Vec::with_capacity(size);
    let mut num_purchases = Vec::with_capacity(size);
    let total_purchases = get_float_rc_vector(size);
    let mut duration_spents = Vec::with_capacity(size);
    let mut duration_sinces = Vec::with_capacity(size);
    let zip_codes = get_stirng_rc_vector(size);
    let mut addresss = Vec::with_capacity(size);
    let mut countrys = Vec::with_capacity(size);
    let mut states = Vec::with_capacity(size);
    let mut first_names = Vec::with_capacity(size);
    let mut last_names = Vec::with_capacity(size);
    let mut provinces = Vec::with_capacity(size);
    let mut comments = Vec::with_capacity(size);
    for i in 0.. size {
        ages.push(Rc::clone(&keys[i]));
        num_purchases.push(Rc::clone(&keys[i]));
        duration_spents.push(Rc::clone(&total_purchases[i]));
        duration_sinces.push(Rc::clone(&total_purchases[i]));
        addresss.push(Rc::clone(&zip_codes[i]));
        countrys.push(Rc::clone(&zip_codes[i]));
        states.push(Rc::clone(&zip_codes[i]));
        first_names.push(Rc::clone(&zip_codes[i]));
        last_names.push(Rc::clone(&zip_codes[i]));
        provinces.push(Rc::clone(&zip_codes[i]));
        comments.push(Rc::clone(&zip_codes[i]));
    }
    // Create vector of CustomerSlice objects and take creation time.
    let (elapsed_create, mut customers) = create_customer_rc_vector(size, &keys, &ages, &num_purchases, &total_purchases, &duration_spents, &duration_sinces, 
        &zip_codes, &addresss, &countrys, &states, &first_names, &last_names, &provinces, &comments, &orders);
    // Access to every feild of each object in the vector and take access time.   
    let elapsed_access = access_rc(& customers);
    let elapsed_drop = drop_rc(&mut customers);
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

