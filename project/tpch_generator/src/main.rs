extern crate tpch_generator;
use tpch_generator::objects::lineitem;
use tpch_generator::objects::order;
use tpch_generator::objects::customer;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let lineitem_file: &str = args.get(1).unwrap();
    let order_file: &str = args.get(2).unwrap();
    let customer_file: &str = args.get(3).unwrap();

    

    

}

fn create_objects_owned(lineitem_file: &str, order_file: &str, customer_file: &str) -> Vec<customer::CustomerOwned>{
    let (_, lineitem_owned_vector) = lineitem::create_lineitem_onwed_vector(lineitem_file);
    let mut agg_lineitems = lineitem::agg_lineitem_by_order_key(lineitem_owned_vector);
    
    let (_, order_owned_vector) = order::get_order_owned_vector(order_file, &mut agg_lineitems);
    let mut agg_orders = order::agg_order_by_custkey(order_owned_vector);

    let (_, customer_owned_vector) = customer::create_customer_onwed_vector(customer_file, &mut agg_orders);
    customer_owned_vector
}


/// Refact impelementation or the way to construct CustomerBorrowed
/// CustomerView can be implemented and performed Read-only operation on that.
fn create_objects_borrowed<'a>(lineitem_file: &str, order_file: &str, customer_file: &str) -> Vec<customer::CustomerBorrowed<'a>>{
    let (_, lineitem_owned_vector) = lineitem::create_lineitem_onwed_vector(lineitem_file);
    let (_, lineitem_borrowed_vector) = lineitem::craete_lineitem_borrowed_vector(&lineitem_owned_vector);
    let mut agg_lineitems_owned = lineitem::agg_lineitem_by_order_key(lineitem_owned_vector);
    let mut agg_lineitems_borrowed = lineitem::agg_lineitem_by_order_key(lineitem_borrowed_vector); 

    let (_, order_owned_vector) = order::get_order_owned_vector(order_file, &mut agg_lineitems_owned);
    let (_ , order_borrowed_vector) = order::get_order_borrowed_vector(&order_owned_vector, &agg_lineitems_borrowed);
    let mut agg_orders_owned = order::agg_order_by_custkey(order_owned_vector);
    let mut agg_orders_borrowed = order::agg_order_by_custkey(order_borrowed_vector);
    
    let (_, customer_owned_vector) = customer::create_customer_onwed_vector(customer_file, &mut agg_orders_owned);
    let (_, customer_borrowed_vector) = customer::create_customer_borrowed_vector(&customer_owned_vector, &agg_orders_borrowed);
    customer_borrowed_vector
}

fn create_objects_rc(lineitem_file: &str, order_file: &str, customer_file: &str) -> Vec<customer::CustomerRc>{
    let (_, lineitem_rc_vector) = lineitem::craete_lineitem_rc_vector(lineitem_file);
    let mut agg_lineitems = lineitem::agg_lineitem_by_order_key(lineitem_rc_vector);
    
    let (_, order_rc_vector) = order::get_order_rc_vector(order_file, &mut agg_lineitems);
    let mut agg_orders = order::agg_order_by_custkey(order_rc_vector);

    let (_, customer_rc_vector) = customer::create_customer_rc_vector(customer_file, &mut agg_orders);
    customer_rc_vector
}


