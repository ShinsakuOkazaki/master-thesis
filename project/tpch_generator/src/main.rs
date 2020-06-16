extern crate tpch_generator;
use tpch_generator::objects::access;
use tpch_generator::objects::customer;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let lineitem_file: &str = args.get(1).unwrap();
    let order_file: &str = args.get(2).unwrap();
    let customer_file: &str = args.get(3).unwrap();
    let lineitem_size: usize = args[4].parse::<usize>().unwrap();
    let order_size: usize = args[5].parse::<usize>().unwrap(); 
    let customer_size: usize = args[6].parse::<usize>().unwrap();
    let (elapsed_owned, customers_owned) = customer::create_objects_owned(&lineitem_file, &order_file, &customer_file, lineitem_size, order_size, customer_size);
    access::access_owned(&customers_owned);
    let (elapsed_borrowed, customers_borrowed) = customer::create_customer_borrowed_vector(&customers_owned);
    access::access_borrowed(&customers_borrowed);
    let (elapsed_borrowed, customers_rc) = customer::create_objects_rc(&lineitem_file, &order_file, &customer_file, lineitem_size, order_size, customer_size);
    access::access_rc(&customers_rc);
    
}


