extern crate tpch_generator;
use tpch_generator::objects::access;
use tpch_generator::objects::customer;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let lineitem_file: &str = args.get(1).unwrap();
    let order_file: &str = args.get(2).unwrap();
    let customer_file: &str = args.get(3).unwrap();

    let (elapsed_owned, customers_owned) = customer::create_objects_owned(&lineitem_file, &order_file, &customer_file);
    access::access_owned(&customers_owned);
    let (elapsed_borrowed, customers_borrowed) = customer::create_customer_borrowed_vector(&customers_owned);
    access::access_borrowed(&customers_borrowed);
    let (elapsed_borrowed, customers_rc) = customer::create_objects_rc(&lineitem_file, &order_file, &customer_file);
    access::access_rc(&customers_rc);
    
}


