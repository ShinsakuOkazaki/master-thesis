use crate::objects::order::*;
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
    orders: &'a Vec<OrderBorrowed<'a>>, 
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
               mktsegment: &'a String, comment: &'a String, orders: &'a Vec<OrderBorrowed>) -> CustomerBorrowed<'a> {


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
pub fn create_customer_onwed_vector(file_name: &str, orders_map: &HashMap<i32, Vec<OrderOwned>>) -> (u128, Vec<CustomerOwned>) {
    let start = Instant::now();
    let path= Path::new(&file_name);
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let mut lines = buf_reader.lines();
    let mut customers = Vec::new();
    for line in lines {
        let l = line.unwrap();
        let row: Vec<&str> = l.split('|').collect();
        let custkey: i32 = row[0].parse::<i32>().unwrap();
        let name: String = row[1].to_string();
        let address: String = row[2].to_string();
        let nationkey: i32 = row[3].parse::<i32>().unwrap();;
        let phone: String = row[4].to_string();
        let acctbal: f64 = row[5].parse::<f64>().unwrap();;
        let mktsegment: String = row[6].to_string();
        let comment: String = row[7].to_string();
        let orders: Vec<OrderOwned> = orders_map.remove(&custkey).unwrap();

        let customer = CustomerOwned::new(custkey, name, address, nationkey, phone, 
                                          acctbal, mktsegment, comment, orders);
        customers.push(customer);
    } 
    let elapsed = start.elapsed().as_micros();
    (elapsed, customers)
}


pub fn create_customer_borrowed_vector<'a>(customers: [CustomerOwned], orders_map: &HashMap<i32, Vec<OrderBorrowed>>) -> (u128, Vec<CustomerBorrowed<'a>>) {
    let start = Instant::now()
    let size = customers.len();
    let mut customers_borrowed: Vec<&CustomerBorrowed> = Vec::new();
    for i in 0..size {
        let customer = customers[i];
        let custkey: i32 = &customer.custkey;
        let name: String = &customer.name;
        let address: String = &customer.address;
        let nationkey: i32 = &customer.nationkey; 
        let phone: String = &customer.phone;
        let acctbal: f64 = &customer.acctbal; 
        let mktsegment: String = &customer.mktsegment;
        let comment: String = &customer.comment;
        let orders: &Vec<OrderBorrowed> = orders_map.get(custkey).unwrap();
        let customer_borrowed = CustomerBorrowed::new(custkey, name, address, nationkey, phone, 
                                          acctbal, mktsegment, comment, orders);
        customers_borrowed.push()
   }
    let elapsed = start.elapsed().as_micros();
    (elapsed, customers)
}

// Function to create a vector of CustomerOwned objects.
pub fn create_customer_rc_vector<'a>(size: usize, keys: &'a Vec<Rc<i32>>, ages: &'a Vec<Rc<i32>>, num_purchases: &'a Vec<Rc<i32>>, 
    total_purchases: &'a Vec<Rc<f64>>, duration_spents: &'a Vec<Rc<f64>>, duration_sinces: &'a Vec<Rc<f64>>,
    zip_codes: &'a Vec<Rc<String>>, addresses: &'a Vec<Rc<String>> , countries: &'a Vec<Rc<String>>,
    states: &'a Vec<Rc<String>>, first_names: &'a Vec<Rc<String>>, last_names: &'a Vec<Rc<String>>,
    provinces: &'a Vec<Rc<String>>, comments: &'a Vec<Rc<String>>, orders: &'a Vec<Rc<OrderRc>>) -> (u128, Vec<CustomerRc>) {
    let start = Instant::now();
    let mut customers: Vec<CustomerRc> = Vec::new();
    for i in 0..size {
        // Get reference by acceesing String in vector and create CustomerBorrowed.
        let key = Rc::clone(&keys[i]);
        let age = Rc::clone(&ages[i]);
        let num_purchase = Rc::clone(&num_purchases[i]);
        let total_purchase = Rc::clone(&total_purchases[i]);
        let duration_spent = Rc::clone(&duration_spents[i]);
        let duration_since = Rc::clone(&duration_sinces[i]);
        let zip_code = Rc::clone(&zip_codes[i]);
        let address = Rc::clone(&addresses[i]);
        let country = Rc::clone(&countries[i]);
        let state = Rc::clone(&states[i]);
        let first_name = Rc::clone(&first_names[i]);
        let last_name = Rc::clone(&last_names[i]);
        let province = Rc::clone(&provinces[i]);
        let comment = Rc::clone(&comments[i]);
        let order = Rc::clone(&orders[i]);
        let customer = CustomerRc::new(key, age, num_purchase, total_purchase, duration_spent, duration_since, 
        zip_code, address, country, state, first_name, last_name, province, comment, order);
        customers.push(customer);
    }
    let elapsed = start.elapsed().as_micros();
    (elapsed, customers)
}


impl Serialize for CustomerOwned {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("CustomerOwned", 6).unwrap();
        state.serialize_field("key", &self.key)?;
        state.serialize_field("age", &self.age)?;
        state.serialize_field("num_purchase", &self.num_purchase)?;
        state.serialize_field("total_purchase", &self.total_purchase)?;
        state.serialize_field("duration_spent", &self.duration_spent)?;
        state.serialize_field("duration_since", &self.duration_since)?;
        state.serialize_field("zip_code", &self.zip_code)?;
        state.serialize_field("address", &self.address)?;
        state.serialize_field("country", &self.country)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("first_name", &self.first_name)?;
        state.serialize_field("last_name", &self.last_name)?;
        state.serialize_field("province", &self.province)?;
        state.serialize_field("comment", &self.comment)?;
        state.serialize_field("order", &self.order)?;
        state.end()
    }
}

impl Serialize for CustomerBorrowed<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("CustomerBorrowed", 6).unwrap();
        state.serialize_field("key", self.key)?;
        state.serialize_field("age", self.age)?;
        state.serialize_field("num_purchase", self.num_purchase)?;
        state.serialize_field("total_purchase", self.total_purchase)?;
        state.serialize_field("duration_spent", self.duration_spent)?;
        state.serialize_field("duration_since", self.duration_since)?;
        state.serialize_field("zip_code", self.zip_code)?;
        state.serialize_field("address", self.address)?;
        state.serialize_field("country", self.country)?;
        state.serialize_field("state", self.state)?;
        state.serialize_field("first_name", self.first_name)?;
        state.serialize_field("last_name", self.last_name)?;
        state.serialize_field("province", self.province)?;
        state.serialize_field("comment", self.comment)?;
        state.serialize_field("order", self.order)?;
        state.end()
    }
}

impl Serialize for CustomerRc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("CustomerRc", 6).unwrap();
        state.serialize_field("key", &self.key)?;
        state.serialize_field("age", &self.age)?;
        state.serialize_field("num_purchase", &self.num_purchase)?;
        state.serialize_field("total_purchase", &self.total_purchase)?;
        state.serialize_field("duration_spent", &self.duration_spent)?;
        state.serialize_field("duration_since", &self.duration_since)?;
        state.serialize_field("zip_code", &self.zip_code)?;
        state.serialize_field("address", &self.address)?;
        state.serialize_field("country", &self.country)?;
        state.serialize_field("state", &self.state)?;
        state.serialize_field("first_name", &self.first_name)?;
        state.serialize_field("last_name", &self.last_name)?;
        state.serialize_field("province", &self.province)?;
        state.serialize_field("comment", &self.comment)?;
        state.serialize_field("order", &self.order)?;
        state.end()
    }
}