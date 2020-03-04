
extern crate rand;
extern crate serde;
extern crate bytes;
use std::env;
use std::sync::Arc;
use std::rc::Rc;
mod objects;
use objects::customer::*;
use objects::order::*;
use objects::field::*;
//use objects::dropping::*;
//use objects::access::*;
use objects::sort::*;
use std::mem;
use std::collections::{VecDeque, LinkedList};


fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let nums1 = get_integer_vector(size);
    println!("Initial: {:?}", nums1);

    let keys = get_integer_vector(size);
    let ages = get_integer_vector(size);
    let num_purchases = get_integer_vector(size);
    let total_purchases = get_float_vector(size);
    let duration_spents = get_float_vector(size);
    let duration_sinces = get_float_vector(size);
    let zip_codes = get_string_vector(size);
    let addresses = get_string_vector(size);
    let countries = get_string_vector(size);
    let states = get_string_vector(size);
    let first_names = get_string_vector(size);
    let last_names = get_string_vector(size);
    let provinces = get_string_vector(size);
    let comments = get_string_vector(size);

    let order_ids = get_integer_vector(size);
    let num_itemss = get_integer_vector(size);
    let payments = get_float_vector(size);
    let order_times = get_float_vector(size);
    let titles = get_string_vector(size);
    let comments_order = get_string_vector(size);

    let mut orders = get_order_owned_vector(size, order_ids, num_itemss, payments,  order_times, titles, comments_order);

    let (_t, mut customers) = create_customer_onwed_vector(size, keys, ages, num_purchases, total_purchases, duration_spents, duration_sinces, 
                                                zip_codes, addresses, countries, states, first_names, last_names, provinces, comments, orders);
    
    let mut atomic_custorms = VecDeque::with_capacity(size);
    for i in 0..size {
        let customer = customers.remove(0);
        atomic_custorms.push_back(Arc::new(customer));
    }
    let sorted_customers = mergesort_vecdeque(atomic_custorms, 0, size);
    print!("cusotomer vector: [");
    for i in 0..size {
        print!("{}, ", sorted_customers[i]);
    }
    print!("]");


    
    

    let keys = get_integer_arc_vector(size);
    let ages = get_integer_arc_vector(size); 
    let num_purchases = get_integer_arc_vector(size); 
    let total_purchases = get_float_arc_vector(size);
    let duration_spents = get_float_arc_vector(size); 
    let duration_sinces = get_float_arc_vector(size); 
    let zip_codes = get_string_arc_vector(size);
    let addresses = get_string_arc_vector(size);
    let countries = get_string_arc_vector(size);
    let states = get_string_arc_vector(size);
    let first_names = get_string_arc_vector(size);
    let last_names = get_string_arc_vector(size);
    let provinces = get_string_arc_vector(size);
    let comments = get_string_arc_vector(size);

    let order_ids = get_integer_arc_vector(size);
    let num_itemss = get_integer_arc_vector(size);
    let payments = get_float_arc_vector(size);
    let order_times = get_float_arc_vector(size);
    let titles = get_string_arc_vector(size);
    let comments_order = get_string_arc_vector(size); 

    let mut orders = get_order_arc_vector(size, &order_ids, &num_itemss, &payments, &order_times, &titles, &comments_order); 
    
    let (_t, mut customers2) = create_customer_arc_vector(size, &keys, &ages, &num_purchases, &total_purchases, &duration_spents, &duration_sinces, 
        &zip_codes, &addresses, &countries, &states, &first_names, &last_names, &provinces, &comments, &orders);
    
    let mut smart_customers2 = Vec::with_capacity(size);
    for i in 0..size {
        smart_customers2.push(Arc::new(customers2.remove(0)));
    }
    let sorted_customers2 = mergesort_mt_mp_gr_nocp(smart_customers2, 0, size);
    print!("cusotomer vector: [");
    for i in 0..size {
        print!("{}, ", sorted_customers2[i]);
    }
    print!("]");

    
    
}