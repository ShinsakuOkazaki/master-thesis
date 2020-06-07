use crate::objects::order::*;
use crate::objects::lineitem::*;
use std::rc::Rc;
use std::time::Instant;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;
// Custorm trait (interface for all objects.)
pub trait Customer {
    fn name_bytes(&self) -> &[u8];
    fn address_bytes(&self)  -> &[u8];
    fn phone_byte(&self)  -> &[u8];
    fn mktsegment_byte(&self) -> &[u8];
    fn comment_byte(&self) -> &[u8];
}



// Objects whose fields are all owned.
pub struct CustomerOwned {
    custkey: i32,
    name: String, 
    address: String, 
    nationkey: i32,
    phone: String, 
    acctbal: f64, 
    mktsegment: String, 
    comment: String,
    orders: Vec<OrderOwned>, 
}

// Objects whose fields are all borrowed.
pub struct CustomerBorrowed<'a> {
    custkey: &'a i32,
    name: &'a String, 
    address: &'a String, 
    nationkey: &'a i32,
    phone: &'a String, 
    acctbal: &'a f64, 
    mktsegment: &'a String, 
    comment: &'a String,
    orders: Vec<OrderBorrowed<'a>>, 
}


pub struct CustomerRc {
    custkey: Rc<i32>,
    name: Rc<String>, 
    address: Rc<String>, 
    nationkey: Rc<i32>,
    phone: Rc<String>, 
    acctbal: Rc<f64>, 
    mktsegment: Rc<String>, 
    comment: Rc<String>,
    orders: Rc<Vec<OrderRc>>, 
}


// Implement new (constructor)
impl CustomerOwned  {
    pub fn new(custkey: i32, name: String, address: String, 
               nationkey: i32, phone: String, acctbal: f64, 
               mktsegment: String, comment: String, orders: Vec<OrderOwned>) -> CustomerOwned {


            CustomerOwned {
                custkey: custkey,
                name: name, 
                address: address, 
                nationkey: nationkey,
                phone: phone, 
                acctbal: acctbal, 
                mktsegment: mktsegment, 
                comment: comment,
                orders: orders, 
            }
    }
}
// Implement new (constructor)
impl CustomerBorrowed<'_>  {
    pub fn new<'a>(custkey: &'a i32, name: &'a String, address: &'a String, 
               nationkey: &'a i32, phone: &'a String, acctbal: &'a f64, 
               mktsegment: &'a String, comment: &'a String, orders: Vec<OrderBorrowed<'a>>) -> CustomerBorrowed<'a> {


            CustomerBorrowed {
                custkey: custkey,
                name: name, 
                address: address, 
                nationkey: nationkey,
                phone: phone, 
                acctbal: acctbal, 
                mktsegment: mktsegment, 
                comment: comment,
                orders: orders, 
            }
    }
}


impl CustomerRc  {
    pub fn new(custkey: Rc<i32>, name: Rc<String>, address: Rc<String>, 
               nationkey: Rc<i32>, phone: Rc<String>, acctbal: Rc<f64>, 
               mktsegment: Rc<String>, comment: Rc<String>, orders: Rc<Vec<OrderRc>>) -> CustomerRc {


            CustomerRc {
                custkey: custkey,
                name: name, 
                address: address, 
                nationkey: nationkey,
                phone: phone, 
                acctbal: acctbal, 
                mktsegment: mktsegment, 
                comment: comment,
                orders: orders, 
            }
    }
}


//Implement Trait to  Struct 
impl Customer for CustomerOwned {

    fn name_bytes(&self) -> &[u8] {
        self.name.as_bytes()
    }
    fn address_bytes(&self)  -> &[u8] {
        self.address.as_bytes()
    }

    fn phone_byte(&self)  -> &[u8]{
        self.phone.as_bytes()
    }

    fn mktsegment_byte(&self) -> &[u8] {
        self.mktsegment.as_bytes()
    }

    fn comment_byte(&self) -> &[u8] {
        self.comment.as_bytes()
    }
}

// Implement Trait to  Struct 
impl Customer for CustomerBorrowed<'_>{

    fn name_bytes(&self) -> &[u8] {
        self.name.as_bytes()
    }
    fn address_bytes(&self)  -> &[u8] {
        self.address.as_bytes()
    }

    fn phone_byte(&self)  -> &[u8]{
        self.phone.as_bytes()
    }

    fn mktsegment_byte(&self) -> &[u8] {
        self.mktsegment.as_bytes()
    }

    fn comment_byte(&self) -> &[u8] {
        self.comment.as_bytes()
    }
}

// Implement Trait to  Struct 
impl Customer for CustomerRc{

    fn name_bytes(&self) -> &[u8] {
        self.name.as_bytes()
    }
    fn address_bytes(&self)  -> &[u8] {
        self.address.as_bytes()
    }

    fn phone_byte(&self)  -> &[u8]{
        self.phone.as_bytes()
    }

    fn mktsegment_byte(&self) -> &[u8] {
        self.mktsegment.as_bytes()
    }

    fn comment_byte(&self) -> &[u8] {
        self.comment.as_bytes()
    }
}

// Function to create a vector of CustomerOwned objects.
pub fn create_customer_onwed_vector(file_name: &str, orders_map: &mut HashMap<i32, Vec<OrderOwned>>) -> Vec<CustomerOwned>{
    let path= Path::new(&file_name);
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let mut customers = Vec::new();
    for line in lines {
        let l = line.unwrap();
        let row: Vec<&str> = l.split('|').collect();
        let custkey: i32 = row[0].parse::<i32>().unwrap();
        let name: String = row[1].to_string();
        let address: String = row[2].to_string();
        let nationkey: i32 = row[3].parse::<i32>().unwrap();
        let phone: String = row[4].to_string();
        let acctbal: f64 = row[5].parse::<f64>().unwrap();
        let mktsegment: String = row[6].to_string();
        let comment: String = row[7].to_string();
        let orders: Vec<OrderOwned> = orders_map.remove(&custkey).unwrap();

        let customer = CustomerOwned::new(custkey, name, address, nationkey, phone, 
                                          acctbal, mktsegment, comment, orders);
        customers.push(customer);
    } 
    customers
}


// pub fn create_customer_borrowed_vector<'a>(customers: &'a [CustomerOwned], orders_map: &'a HashMap<i32, Vec<OrderBorrowed>>) -> (u128, Vec<CustomerBorrowed<'a>>) {
//     let start = Instant::now();
//     let size = customers.len();
//     let mut customers_borrowed: Vec<CustomerBorrowed> = Vec::new();
//     for i in 0..size {
//         let customer = customers.get(i).unwrap();
//         let custkey = &customer.custkey;
//         let name = &customer.name;
//         let address = &customer.address;
//         let nationkey= &customer.nationkey; 
//         let phone= &customer.phone;
//         let acctbal= &customer.acctbal; 
//         let mktsegment= &customer.mktsegment;
//         let comment= &customer.comment;
//         let orders= orders_map.get(custkey).unwrap();
//         let customer_borrowed = CustomerBorrowed::new(custkey, name, address, nationkey, phone, 
//                                           acctbal, mktsegment, comment, orders);
//         customers_borrowed.push(customer_borrowed);
//    }
//     let elapsed = start.elapsed().as_micros();
//     (elapsed, customers_borrowed)
// }

// Function to create a vector of CustomerOwned objects.
pub fn create_customer_rc_vector(file_name: &str, orders_map: &mut HashMap<i32, Vec<OrderRc>>) -> Vec<CustomerRc>{
    let path= Path::new(&file_name);
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let mut customers = Vec::new();
    for line in lines {
        let l = line.unwrap();
        let row: Vec<&str> = l.split('|').collect();
        let custkey= Rc::new(row[0].parse::<i32>().unwrap());
        let name= Rc::new(row[1].to_string());
        let address= Rc::new(row[2].to_string());
        let nationkey= Rc::new(row[3].parse::<i32>().unwrap());
        let phone= Rc::new(row[4].to_string());
        let acctbal= Rc::new(row[5].parse::<f64>().unwrap());
        let mktsegment= Rc::new(row[6].to_string());
        let comment= Rc::new(row[7].to_string());
        let orders= Rc::new(orders_map.remove(&custkey).unwrap());

        let customer = CustomerRc::new(custkey, name, address, nationkey, phone, 
                                          acctbal, mktsegment, comment, orders);
        customers.push(customer);
    } 
    customers
}


impl Serialize for CustomerOwned {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("CustomerOwned", 6).unwrap();
        state.serialize_field("custkey", &self.custkey)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("address", &self.address)?;
        state.serialize_field("nationkey", &self.nationkey)?;
        state.serialize_field("phone", &self.phone)?;
        state.serialize_field("acctbal", &self.acctbal)?;
        state.serialize_field("mktsegment", &self.mktsegment)?;
        state.serialize_field("comment", &self.comment)?;
        state.serialize_field("orders", &self.orders)?;
        state.end()
    }
}

impl Serialize for CustomerBorrowed<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("CustomerBorrowed", 6).unwrap();
        state.serialize_field("custkey", &self.custkey)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("address", &self.address)?;
        state.serialize_field("nationkey", &self.nationkey)?;
        state.serialize_field("phone", &self.phone)?;
        state.serialize_field("acctbal", &self.acctbal)?;
        state.serialize_field("mktsegment", &self.mktsegment)?;
        state.serialize_field("comment", &self.comment)?;
        state.serialize_field("orders", &self.orders)?;
        state.end()
    }
}

impl Serialize for CustomerRc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("CustomerRc", 6).unwrap();
        state.serialize_field("custkey", &self.custkey)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("address", &self.address)?;
        state.serialize_field("nationkey", &self.nationkey)?;
        state.serialize_field("phone", &self.phone)?;
        state.serialize_field("acctbal", &self.acctbal)?;
        state.serialize_field("mktsegment", &self.mktsegment)?;
        state.serialize_field("comment", &self.comment)?;
        state.serialize_field("orders", &self.orders)?;
        state.end()
    }
}



pub fn create_customer_borrowed_from_owned<'a>(customer_owned: &'a CustomerOwned) -> CustomerBorrowed<'a> {
    let orders_borrowed = get_order_borrowed_vector(&customer_owned.orders);
    let customer_borrowed = CustomerBorrowed::new(&customer_owned.custkey, &customer_owned.name, &customer_owned.address, &customer_owned.nationkey, &customer_owned.phone, &customer_owned.acctbal, 
                                     &customer_owned.mktsegment, &customer_owned.comment, orders_borrowed);
    customer_borrowed
}

pub fn create_customer_borrowed_vector<'a>(customers_owned: &'a [CustomerOwned]) -> Vec<CustomerBorrowed<'a>>{
    let size = customers_owned.len();
    let mut customers_borrowed = Vec::with_capacity(size);

    for i in 0..size {
        let customer_borrowed = create_customer_borrowed_from_owned(&customers_owned[i]);
        customers_borrowed.push(customer_borrowed);
    }
    customers_borrowed
}

pub fn create_objects_owned(lineitem_file: &str, order_file: &str, customer_file: &str) -> (u128, Vec<CustomerOwned>){
    let start = Instant::now();
    let lineitem_owned_vector = create_lineitem_onwed_vector(lineitem_file);
    let mut agg_lineitems = agg_lineitem_by_order_key(lineitem_owned_vector);
    
    let order_owned_vector = get_order_owned_vector(order_file, &mut agg_lineitems);
    let mut agg_orders = agg_order_by_custkey(order_owned_vector);

    let customer_owned_vector = create_customer_onwed_vector(customer_file, &mut agg_orders);
    let elapsed = start.elapsed().as_micros();
    (elapsed, customer_owned_vector)
}

pub fn create_objects_rc(lineitem_file: &str, order_file: &str, customer_file: &str) -> (u128, Vec<CustomerRc>){
    let start = Instant::now();
    let lineitem_rc_vector= craete_lineitem_rc_vector(lineitem_file);
    let mut agg_lineitems = agg_lineitem_by_order_key(lineitem_rc_vector);
    
    let order_rc_vector = get_order_rc_vector(order_file, &mut agg_lineitems);
    let mut agg_orders = agg_order_by_custkey(order_rc_vector);

    let customer_rc_vector = create_customer_rc_vector(customer_file, &mut agg_orders);
    let elapsed = start.elapsed().as_micros();
    (elapsed, customer_rc_vector)
}

