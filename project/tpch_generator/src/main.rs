extern crate tpch_generator;
use tpch_generator::objects;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let lineitem_file: &str = args.get(1).unwrap();
    //let order_file: String = args[2];
    //let customer_file: String = args[3];

    let (_, lineitem_owned_vector) = objects::lineitem::create_lineitem_onwed_vector(lineitem_file);
    let agg_lineitems = objects::lineitem::agg_lineitem_by_order_key(lineitem_owned_vector);
    for (key, value) in agg_lineitems.iter() {
        println!("{:?}", key);
    }
    // let order_owned_vector = objects::order::get_order_owned_vector(&order_file, line_items_map);
    // let customer_owned_vector = objects::customer::create_customer_onwed_vector(file_name, orders_map);
}
