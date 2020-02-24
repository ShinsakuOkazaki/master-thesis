
extern crate rand;
extern crate serde;
extern crate bytes;
use std::env;
use std::sync::Arc;
mod objects;
use objects::customer::{create_customer_onwed_vector};
use objects::order::{get_order_owned_vector};
use objects::field::{get_float_vector, get_integer_vector, get_string_vector};
//use objects::dropping::*;
//use objects::access::*;
use objects::sort::*;


fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let nums1 = get_integer_vector(size);
    println!("Initial: {:?}", nums1);

    let nums2 = get_integer_vector(size);
    let n2 = nums2.len();
    let merged = mergesort_mt_mp(nums2, 0, n2);
    println!("Sort with MT{:?}", merged);

    let nums3 = get_integer_vector(size);
    let n3 = nums3.len();
    let num3_atomic = Arc::new(Mutex::new(nums3));
    mergesort_mt_mutex(Arc::clone(&num3_atomic), 0, n3);
    println!("Sort with Mutex{:?}", num3_atomic);

    let nums4 = get_integer_vector(size);
    let n4 = nums4.len();
    let num4_atomic = Arc::new(nums4);
    let merged2 = mergesort_mt_im(Arc::clone(&num4_atomic), 0, n4);
    println!("Sort with Immutable{:?}", merged2);
    
    let nums5 = get_integer_vector(size);
    let n5= nums5.len();
    //let num5_atomic = Arc::new(nums5);
    let merged5 = mergesort_mt_mp_improve(nums5, 0, n5);
    println!("Sort with Improve{:?}", merged5);

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

    let (_t, customers) = create_customer_onwed_vector(size, keys, ages, num_purchases, total_purchases, duration_spents, duration_sinces, 
                                                zip_codes, addresses, countries, states, first_names, last_names, provinces, comments, orders);
    let num_customer = customers.len();
    let sorted_customer = mergesort_mt_mp_customer(customers, 0, num_customer);

    print!("cusotomer vector: [");
    for i in 0..sorted_customer.len() {
        print!("{}, ", sorted_customer[i]);
    }
    print!("]");
    
}