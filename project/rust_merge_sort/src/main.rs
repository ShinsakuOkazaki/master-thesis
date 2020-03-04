
extern crate rand;
extern crate serde;
extern crate bytes;
use std::env;
mod objects;
use objects::customer::create_customer_onwed_vector;
use objects::order::get_order_owned_vector;
use objects::field::*;
//use objects::dropping::*;
//use objects::access::*;
use objects::sort::*;
//use std::mem;
use std::collections::{VecDeque, LinkedList};
use std::fs::OpenOptions;
use std::io::prelude::*;
use objects::access::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let method: i32 = args[2].parse().unwrap(); 
    run(method, size);
}

fn run(method: i32, size: usize) {
    match method {
        1 => run_ex_shared(size),
        2 => run_ex_scope(size),
        3 => run_ex_linkedlist(size),
        _ => println!("Wrong input!")
    }
}

fn run_ex_shared(size: usize) {
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

    let (_, customers) = create_customer_onwed_vector(size, keys, ages, num_purchases, total_purchases, duration_spents, duration_sinces, 
                                                                    zip_codes, addresss, countrys, states, first_names, last_names, provinces, comments, orders);

    let q_custorms = VecDeque::from(customers);
    
    let (elapsed_sort, sorted) = mergesort_vecdeque(q_custorms, 0, size);
    let (slice, _) = sorted.as_slices();
    access_owned(slice);
    write_to_file(size, "shared", elapsed_sort);
    
}


fn run_ex_scope(size: usize) {
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

    let (_, customers) = create_customer_onwed_vector(size, keys, ages, num_purchases, total_purchases, duration_spents, duration_sinces, 
                                                                    zip_codes, addresss, countrys, states, first_names, last_names, provinces, comments, orders);

    let len = customers.len();
    let (elapsed_sort, sorted) = mergesort_vecdeque_slice(customers, len);
    let (slice, _) = sorted.as_slices();
    access_owned(slice); 
    write_to_file(size, "scope", elapsed_sort);
}


fn run_ex_linkedlist(size: usize) {
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

    let (_, mut customers) = create_customer_onwed_vector(size, keys, ages, num_purchases, total_purchases, duration_spents, duration_sinces, 
                                                                    zip_codes, addresss, countrys, states, first_names, last_names, provinces, comments, orders);
    

    let mut head = LinkedList::new();
    for _ in 0..size {
        head.push_back(customers.remove(0));
    }
    let (elapsed_sort, sorted) = mergesort_linkedlist(head);
    access_linkedlist(&sorted);
    write_to_file(size, "linkedlist", elapsed_sort);
}


fn write_to_file(size: usize, method: &str, elapsed_sort: u128) {
    let output = format!("[RustVector]#{:?}#{:?}#{:?}\n", 
                         size, method, elapsed_sort);
    println!("{}",output);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");
}
