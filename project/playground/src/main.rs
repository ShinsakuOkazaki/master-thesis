
extern crate rand;
extern crate serde;
extern crate bytes;
use std::env;
mod objects;
use objects::customer::{create_customer_onwed_vector, Customer};
use objects::order::get_order_owned_vector;
use objects::field::*;
//use objects::dropping::*;
//use objects::access::*;
use objects::sort::*;
//use std::mem;
use std::collections::VecDeque;
//use std::collections::LinkedList;
use std::fs::OpenOptions;
use std::io::prelude::*;
use objects::access::*;
use objects::aggregate::*;
use std::path::Path;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let num_partition: usize = args[2].parse().unwrap();
    let method: i32 = args[3].parse().unwrap();
    if size < num_partition {
        println!("Size and number of partition!");
    } else {
        run(method, size, num_partition);     
    }
}

fn run(method: i32, size: usize, num_partition: usize) {
    match method {
        1 => run_ex_shared(size, num_partition),
       // 2 => run_ex_copy(size),
        _ => println!("Wrong input!")
    }
}

fn run_ex_shared(size: usize, num_partition: usize) {
    println!("running");
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
    
    
    let bigger_size: usize = size / num_partition + 1;
    let bigger_num: usize = size % num_partition;
    println!("bigger_size: {}", bigger_size);
    println!("bigger_num: {}", bigger_num); 
    let smaller_size: usize = size / num_partition;
    let smaller_num : usize=  num_partition - size % num_partition;
    println!("smaller_size: {}", smaller_size);
    println!("smaler_num: {}", smaller_num);
    let mut partitions = Vec::with_capacity(num_partition);
    for i in 0..bigger_num {
        let lower_bound = i * bigger_size;
        let upper_bound = (i + 1)* bigger_size;
        println!("lower_bound: {}", lower_bound);
        println!("upper_bound: {}", upper_bound);
        let customers_with_arc = wrap_elements_with_arc(&customers[lower_bound..upper_bound]);
        let path = format!("/partition/partition_{}.txt", i);
        println!("{}", path);
        partitions.push(path);
        println!("pushed");
        serialize_vector_arc(customers_with_arc, &partitions[i]);
    }

    for i in 0..smaller_num {
        let offset = bigger_size * bigger_num;
        let lower_bound = offset + i * smaller_size;
        let upper_bound = offset + (i + 1) * smaller_size;
        let customers_with_arc = wrap_elements_with_arc(&customers[lower_bound..upper_bound]);
        let path = format!("/partition/partition_{}.txt", i + bigger_num);
        println!("{}", path);
        partitions.push(path);
        serialize_vector_arc(customers_with_arc, &partitions[i + bigger_num]);
    }
    let partitions_arc = Arc::new(partitions);
    let (elapsed_sort, aggregated) = tree_aggregate_run(Arc::clone(&partitions_arc));
    write_to_file(size, num_partition, "shared", elapsed_sort);
}


// fn run_ex_copy(size: usize) {
    
// }

fn wrap_elements_with_arc<T>(customers: &[T]) -> Vec<Arc<T>>
    where T: Customer + Clone
{
    let n = customers.len();
    let mut res = Vec::with_capacity(n);
    for i in 0..n {
        res.push(Arc::new(customers[i].clone()));
    }
    return res;
}


fn write_to_file(size: usize, num_partition: usize, method: &str, elapsed_sort: u128) {
    let output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}\n", 
                         size, num_partition, method, elapsed_sort);
    println!("{}",output);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");
}