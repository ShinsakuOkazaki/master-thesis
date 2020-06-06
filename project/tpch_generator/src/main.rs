extern crate tpch_generator;
use tpch_generator::objects::customer;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let lineitem_file: &str = args.get(1).unwrap();
    let order_file: &str = args.get(2).unwrap();
    let customer_file: &str = args.get(3).unwrap();

    let customers_owned = customer::create_objects_owned(&lineitem_file, &order_file, &customer_file);
    let customers_borrowed = customer::create_customer_borrowed_vector(&customers_owned);
    let customers_rc = customer::create_objects_rc(&lineitem_file, &order_file, &customer_file);

}