use std::rc::Rc;
use std::time::Instant;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fs::File;
use crate::objects::lineitem::*;
use std::collections::HashMap;

pub trait Order {
    fn get_custkey(&mut self) -> i32;
}

pub struct OrderOwned {
    order_key: i32,
    custkey: i32,
    order_status: String,
    total_price: f64,
    order_date: String,
    order_priority: String, 
    clerk: String,
    shippriority: i32,
    comment: String, 
    line_items: Vec<LineItemOwned>, 
}


pub struct OrderBorrowed<'a> {
    order_key: &'a i32,
    custkey: &'a i32,
    order_status: &'a String,
    total_price: &'a f64,
    order_date: &'a String,
    order_priority: &'a String, 
    clerk: &'a String,
    shippriority: &'a i32,
    comment: &'a String, 
    line_items: Vec<LineItemBorrowed<'a>>,
}

pub struct OrderRc {
    order_key: Rc<i32>,
    custkey: Rc<i32>,
    order_status: Rc<String>,
    total_price: Rc<f64>,
    order_date: Rc<String>,
    order_priority: Rc<String>,  
    clerk: Rc<String>,
    shippriority: Rc<i32>,
    comment: Rc<String> ,
    line_items: Rc<Vec<LineItemRc>>, 
}

impl OrderOwned {
    pub fn new(order_key: i32, custkey: i32, order_status: String, total_price: f64, order_date: String, 
               order_priority: String, clerk: String, shippriority: i32, comment: String, line_items: Vec<LineItemOwned>) -> OrderOwned {
        OrderOwned {
            order_key: order_key,
            custkey: custkey,
            order_status: order_status,
            total_price: total_price,
            order_date: order_date,
            order_priority: order_priority, 
            clerk: clerk,
            shippriority: shippriority,
            comment: comment,
            line_items: line_items,
        }
    }
}

impl OrderBorrowed<'_> {
    pub fn new<'a>(order_key: &'a i32, custkey: &'a i32, order_status: &'a String, total_price: &'a f64, order_date: &'a String, 
                   order_priority: &'a String, clerk: &'a String, shippriority: &'a i32, comment: &'a String, line_items: Vec<LineItemBorrowed<'a>>) -> OrderBorrowed<'a> { 
        OrderBorrowed {
           order_key: order_key,
            custkey: custkey,
            order_status: order_status,
            total_price: total_price,
            order_date: order_date,
            order_priority: order_priority, 
            clerk: clerk,
            shippriority: shippriority,
            comment: comment, 
            line_items: line_items, 
        }
    }
}

impl OrderRc {
    pub fn new(order_key: Rc<i32>, custkey: Rc<i32>, order_status: Rc<String>, total_price: Rc<f64>, order_date: Rc<String>,
    order_priority: Rc<String>,  clerk: Rc<String>, shippriority: Rc<i32>, comment: Rc<String>, line_items: Rc<Vec<LineItemRc>>) -> OrderRc {
        OrderRc {
            order_key: order_key,
            custkey: custkey,
            order_status: order_status,
            total_price: total_price,
            order_date: order_date,
            order_priority: order_priority, 
            clerk: clerk,
            shippriority: shippriority,
            comment: comment, 
            line_items: line_items,
        }
    }
}


impl Order for OrderOwned {
    fn get_custkey(&mut self) -> i32 {
        self.custkey
    }
}

impl Order for OrderBorrowed<'_> {
    fn get_custkey(&mut self) -> i32 {
        let res = *self.custkey;
        res
    }
}

impl Order for OrderRc {
    fn get_custkey(&mut self) -> i32 {
        let res = *Rc::get_mut(&mut self.custkey).unwrap();
        res
    }
}

pub fn get_order_owned_vector(file_name: &str, line_items_map: &mut HashMap<i32, Vec<LineItemOwned>>) -> (u128, Vec<OrderOwned>) {
    
    let start = Instant::now();
    let path= Path::new(&file_name);
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let mut orders = Vec::new();
    for line in lines {
        let l = line.unwrap();
        let row: Vec<&str> = l.split('|').collect();
        let order_key: i32 = row[0].parse::<i32>().unwrap();
        let custkey: i32 = row[1].parse::<i32>().unwrap();
        let order_status: String = row[2].to_string();
        let total_price: f64 = row[3].parse::<f64>().unwrap();
        let order_date: String = row[4].to_string() ;
        let order_priority: String = row[5].to_string();
        let clerk: String = row[6].to_string();
        let shippriority: i32 = row[7].parse::<i32>().unwrap(); 
        let comment: String = row[8].parse::<String>().unwrap();
        let line_items: Vec<LineItemOwned> = line_items_map.remove(&order_key).unwrap();

        let order = OrderOwned::new(order_key, custkey, order_status, total_price,
                                    order_date, order_priority, clerk, shippriority, comment, line_items);
        
        orders.push(order);
    }
    let elapsed = start.elapsed().as_micros();
    (elapsed, orders)
}

// pub fn get_order_borrowed_vector<'a>(orders_owned: &'a [OrderOwned], line_items_map: &'a HashMap<i32, Vec<LineItemBorrowed<'a>>>) -> (u128, Vec<OrderBorrowed<'a>>) {
//     let size = orders_owned.len();
//     let start = Instant::now();
//     let mut orders_borrowed = Vec::with_capacity(size);

//     for i in 0..size {
//         let order_owned: &'a OrderOwned = orders_owned.get(i).unwrap();
//         let order_key = &order_owned.order_key;
//         let custkey = &order_owned.custkey;
//         let order_status = &order_owned.order_status;
//         let total_price = &order_owned.total_price;
//         let order_date = &order_owned.order_date;
//         let order_priority = &order_owned.order_priority;
//         let clerk = &order_owned.clerk;
//         let shippriority = &order_owned.shippriority;
//         let comment = &order_owned.comment;
//         let line_items = line_items_map.get(&order_key).unwrap();
        
//         let order_borrowed = OrderBorrowed::new(order_key, custkey, order_status, total_price,
//                                     order_date, order_priority, clerk, shippriority, comment, line_items);

//         orders_borrowed.push(order_borrowed);
//     }
//     let elapsed = start.elapsed().as_micros();
//     (elapsed, orders_borrowed)
// }

pub fn get_order_rc_vector(file_name: &str, line_items_map: &mut HashMap<i32, Vec<LineItemRc>>) -> (u128, Vec<OrderRc>) {
    
    let start = Instant::now();
    let path= Path::new(&file_name);
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let mut orders = Vec::new();
    for line in lines {
        let l = line.unwrap();
        let row: Vec<&str> = l.split('|').collect();
        let order_key: Rc<i32> = Rc::new(row[0].parse::<i32>().unwrap());
        let custkey: Rc<i32> = Rc::new(row[1].parse::<i32>().unwrap());
        let order_status: Rc<String> = Rc::new(row[2].to_string());
        let total_price: Rc<f64> = Rc::new(row[3].parse::<f64>().unwrap());
        let order_date: Rc<String> = Rc::new(row[4].to_string() );
        let order_priority: Rc<String> = Rc::new(row[5].to_string());
        let clerk: Rc<String> = Rc::new(row[6].to_string());
        let shippriority: Rc<i32> = Rc::new(row[7].parse::<i32>().unwrap()); 
        let comment: Rc<String> = Rc::new(row[8].parse::<String>().unwrap());
        let line_items: Rc<Vec<LineItemRc>> = Rc::new(line_items_map.remove(&order_key).unwrap());

        let order = OrderRc::new(order_key, custkey, order_status, total_price,
                                    order_date, order_priority, clerk, shippriority, comment, line_items);
        
        orders.push(order);
    }
    let elapsed = start.elapsed().as_micros();
    (elapsed, orders)
}

pub fn get_order_borrowed_from_owned<'a>(order_owned: &'a OrderOwned) -> OrderBorrowed<'a> {
    let lineitems_borrowed = get_lineitems_borrowed(&order_owned.line_items);
    let order_borrowed = OrderBorrowed::new(&order_owned.order_key, &order_owned.custkey, &order_owned.order_status, &order_owned.total_price,
                                    &order_owned.order_date, &order_owned.order_priority, &order_owned.clerk, &order_owned.shippriority, &order_owned.comment, lineitems_borrowed);
    order_borrowed
}

pub fn get_order_borrowed_vector<'a>(orders_owned: &'a [OrderOwned]) -> Vec<OrderBorrowed<'a>>{
    let size = orders_owned.len();
    let mut orders_borrowed = Vec::with_capacity(size);

    for i in 0..size {
        let order_borrowed = get_order_borrowed_from_owned(&orders_owned[i]);
        orders_borrowed.push(order_borrowed);
    }
    orders_borrowed
}

impl Serialize for OrderOwned {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("OrderOwned", 10)?;
        state.serialize_field("order_key", &self.order_key)?;
        state.serialize_field("custkey", &self.custkey)?;
        state.serialize_field("order_status", &self.order_status)?;
        state.serialize_field("total_price", &self.total_price)?;
        state.serialize_field("order_date", &self.order_date)?;
        state.serialize_field("order_priority", &self.order_priority)?;
        state.serialize_field("clerk", &self.clerk)?;
        state.serialize_field("shippriority", &self.shippriority)?;
        state.serialize_field("comment", &self.comment)?;
        state.serialize_field("line_items", &self.line_items)?;
        state.end()
    }
}


impl Serialize for OrderBorrowed<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("OrderOwned", 10)?;
        state.serialize_field("order_key", &self.order_key)?;
        state.serialize_field("custkey", &self.custkey)?;
        state.serialize_field("order_status", &self.order_status)?;
        state.serialize_field("total_price", &self.total_price)?;
        state.serialize_field("order_date", &self.order_date)?;
        state.serialize_field("order_priority", &self.order_priority)?;
        state.serialize_field("clerk", &self.clerk)?;
        state.serialize_field("shippriority", &self.shippriority)?;
        state.serialize_field("comment", &self.comment)?;
        state.serialize_field("line_items", &self.line_items)?;
        state.end()
    }
}

impl Serialize for OrderRc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("OrderOwned", 10)?;
        state.serialize_field("order_key", &self.order_key)?;
        state.serialize_field("custkey", &self.custkey)?;
        state.serialize_field("order_status", &self.order_status)?;
        state.serialize_field("total_price", &self.total_price)?;
        state.serialize_field("order_date", &self.order_date)?;
        state.serialize_field("order_priority", &self.order_priority)?;
        state.serialize_field("clerk", &self.clerk)?;
        state.serialize_field("shippriority", &self.shippriority)?;
        state.serialize_field("comment", &self.comment)?;
        state.serialize_field("line_items", &self.line_items)?;
        state.end()
    }
}


pub fn agg_order_by_custkey<T: Order>(mut orders: Vec<T>) -> HashMap<i32, Vec<T>> {
    let size = orders.len();
    let mut aggregation: HashMap<i32, Vec<T>> = HashMap::new();
    
    for _ in 0..size {
        let mut order = orders.pop().unwrap();
        let key = order.get_custkey();
        if !aggregation.contains_key(&key) {
            aggregation.insert(key, Vec::new());
        } 
        aggregation.get_mut(&key).unwrap().push(order);
    }

    aggregation
}