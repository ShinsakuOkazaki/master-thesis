use crate::objects::order::{OrderOwned, OrderBorrowed, OrderRc};
use std::rc::Rc;
use std::time::Instant;
use std::cmp::Ordering;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::fmt;
use std::marker::{Send, Sync};
// Custorm trait (interface for all objects.)
pub trait Customer {
    fn zip_code_bytes(&self) -> &[u8];
    fn address_bytes(&self)  -> &[u8];
    fn country_bytes(&self)  -> &[u8];
}



// Objects whose fields are all owned.
#[derive(Clone, Debug)]
pub struct CustomerOwned {
    key: i32,
    age: i32,
    num_purchase: i32,
    total_purchase: f64,
    duration_spent: f64, 
    duration_since: f64,
    zip_code: String,
    address: String,
    country: String,
    state: String,
    first_name: String,
    last_name: String,
    province: String,
    comment: String, 
    order: OrderOwned
}

// Objects whose fields are all borrowed.
#[derive(Clone, Debug)]
pub struct CustomerBorrowed<'a> {
    key: &'a i32,
    age: &'a i32,
    num_purchase: &'a i32,
    total_purchase: &'a f64,
    duration_spent: &'a f64, 
    duration_since: &'a f64,
    zip_code: &'a String,
    address: &'a String,
    country: &'a String,
    state: &'a String,
    first_name: &'a String,
    last_name: &'a String,
    province: &'a String,
    comment: &'a String, 
    order: &'a OrderBorrowed<'a>
}
#[derive(Clone, Debug)]
pub struct CustomerRc {
    key: Rc<i32>,
    age: Rc<i32>,
    num_purchase: Rc<i32>,
    total_purchase: Rc<f64>,
    duration_spent: Rc<f64>, 
    duration_since: Rc<f64>,
    zip_code: Rc<String>,
    address: Rc<String>,
    country: Rc<String>,
    state: Rc<String>,
    first_name: Rc<String>,
    last_name: Rc<String>,
    province: Rc<String>,
    comment: Rc<String>, 
    order: Rc<OrderRc>
}


// Implement new (constructor)
impl CustomerOwned  {
    pub fn new(key: i32, age: i32, num_purchase: i32, total_purchase: f64, duration_spent: f64, duration_since: f64, 
               zip_code: String, address: String, country: String, state: String, 
               first_name: String, last_name: String, province: String, comment: String, order: OrderOwned) -> CustomerOwned {

        CustomerOwned {
            key: key, 
            age: age,
            num_purchase: num_purchase, 
            total_purchase: total_purchase,
            duration_spent: duration_spent,
            duration_since: duration_since,
            zip_code: zip_code,
            address: address,
            country: country,
            state: state,
            first_name: first_name, 
            last_name: last_name, 
            province: province,
            comment: comment,
            order: order
        }
    }
}
// Implement new (constructor)
impl CustomerBorrowed<'_> {
    pub fn new<'a>(key: &'a i32, age: &'a i32, num_purchase: &'a i32, total_purchase: &'a f64, duration_spent: &'a f64, duration_since: &'a f64, 
                    zip_code: &'a String, address: &'a String, country: &'a String, state: &'a String, 
                    first_name: &'a String, last_name: &'a String, province: &'a String, comment: &'a String, order: &'a OrderBorrowed) -> CustomerBorrowed<'a> {
        CustomerBorrowed {
            key: key, 
            age: age,
            num_purchase: num_purchase, 
            total_purchase: total_purchase,
            duration_spent: duration_spent,
            duration_since: duration_since,
            zip_code: zip_code,
            address: address,
            country: country,
            state: state,
            first_name: first_name, 
            last_name: last_name, 
            province: province,
            comment: comment,
            order: order
        }
    }
}


impl CustomerRc {
    pub fn new(key: Rc<i32>, age: Rc<i32>, num_purchase: Rc<i32>, total_purchase: Rc<f64>, duration_spent: Rc<f64>, duration_since: Rc<f64>, 
        zip_code: Rc<String>, address: Rc<String>, country: Rc<String>, state: Rc<String>, 
        first_name: Rc<String>, last_name: Rc<String>, province: Rc<String>, comment: Rc<String>, order: Rc<OrderRc>) -> CustomerRc{
        CustomerRc {
            key: key, 
            age: age,
            num_purchase: num_purchase, 
            total_purchase: total_purchase,
            duration_spent: duration_spent,
            duration_since: duration_since,
            zip_code: zip_code,
            address: address,
            country: country,
            state: state,
            first_name: first_name, 
            last_name: last_name, 
            province: province,
            comment: comment,
            order: order
        }
    }
}


//Implement Trait to  Struct 
impl Customer for CustomerOwned {
    fn zip_code_bytes(&self) -> &[u8] {
        self.zip_code.as_bytes()
    }
    fn address_bytes(&self) -> &[u8] {
        self.address.as_bytes()
    }
    fn country_bytes(&self) -> &[u8]{
        self.country.as_bytes()
    }
}

// Implement Trait to  Struct 
impl Customer for CustomerBorrowed<'_>{
    fn zip_code_bytes(&self) -> &[u8] {
        self.zip_code.as_bytes()
    }
    fn address_bytes(&self) -> &[u8] {
        self.address.as_bytes()
    }
    fn country_bytes(&self) -> &[u8] {
        self.country.as_bytes()
    }
}

// Implement Trait to  Struct 
impl Customer for CustomerRc {
    fn zip_code_bytes(&self) -> &[u8] {
        self.zip_code.as_bytes()
    }
    fn address_bytes(&self) -> &[u8] {
        self.address.as_bytes()
    }
    fn country_bytes(&self) -> &[u8] {
        self.country.as_bytes()
    }
}

impl Eq for CustomerOwned {}

impl Ord for CustomerOwned {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for CustomerOwned {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CustomerOwned {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for CustomerBorrowed<'_> {}

impl Ord for CustomerBorrowed<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for CustomerBorrowed<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CustomerBorrowed<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl Eq for CustomerRc {}

impl Ord for CustomerRc {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}

impl PartialOrd for CustomerRc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for CustomerRc {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl fmt::Display for CustomerOwned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", self.key)
    }
}

impl fmt::Display for CustomerBorrowed<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", self.key)
    }
}

impl fmt::Display for CustomerRc {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", self.key)
    }
}

unsafe impl Send for CustomerOwned {}
unsafe impl Send for CustomerBorrowed<'_> {}
unsafe impl Send for CustomerRc {}

unsafe impl Sync for CustomerOwned {}
unsafe impl Sync for CustomerBorrowed<'_> {}
unsafe impl Sync for CustomerRc {}


// Function to create a vector of CustomerOwned objects.
pub fn create_customer_onwed_vector(size: usize, mut keys: Vec<i32>, mut ages: Vec<i32>, mut num_purchases: Vec<i32>, 
        mut total_purchases: Vec<f64>, mut duration_spents: Vec<f64>, mut duration_sinces: Vec<f64>,
        mut zip_codes: Vec<String>, mut addresses: Vec<String> ,mut countries: Vec<String>,
        mut states: Vec<String>, mut first_names: Vec<String>, mut last_names: Vec<String>,
        mut provinces: Vec<String>, mut comments: Vec<String>, mut orders: Vec<OrderOwned>) -> (u128, Vec<CustomerOwned>) {
    let start = Instant::now();
    let mut customers: Vec<CustomerOwned> = Vec::new();
    for _ in 0..size {
        // Get owner by poping String from vector and create CustomerOwned.
        let key = keys.pop().unwrap();
        let age = ages.pop().unwrap();
        let num_purchase = num_purchases.pop().unwrap();
        let total_purchase = total_purchases.pop().unwrap();
        let duration_spent = duration_spents.pop().unwrap();
        let duration_since = duration_sinces.pop().unwrap();
        let zip_code = zip_codes.pop().unwrap();
        let address = addresses.pop().unwrap();
        let country = countries.pop().unwrap();
        let state = states.pop().unwrap();
        let first_name = first_names.pop().unwrap();
        let last_name = last_names.pop().unwrap();
        let province = provinces.pop().unwrap();
        let comment = comments.pop().unwrap();
        let order = orders.pop().unwrap();
        let customer = CustomerOwned::new(key, age, num_purchase, total_purchase, duration_spent, duration_since, 
                    zip_code, address, country, state, first_name, last_name, province, comment, order);
        customers.push(customer);
    }
    let elapsed = start.elapsed().as_millis();
    (elapsed, customers)
}

pub fn create_customer_borrowed_vector<'a>(size: usize, keys: &'a Vec<i32>, ages: &'a Vec<i32>, num_purchases: &'a Vec<i32>, 
                total_purchases: &'a Vec<f64>, duration_spents: &'a Vec<f64>, duration_sinces: &'a Vec<f64>,
                zip_codes: &'a Vec<String>, addresses: &'a Vec<String> , countries: &'a Vec<String>,
                states: &'a Vec<String>, first_names: &'a Vec<String>, last_names: &'a Vec<String>,
                provinces: &'a Vec<String>, comments: &'a Vec<String>, orders: &'a Vec<OrderBorrowed>) -> (u128, Vec<CustomerBorrowed<'a>>) {
    let start = Instant::now();
    let mut customers: Vec<CustomerBorrowed> = Vec::new();
    for i in 0..size {
        // Get reference by acceesing String in vector and create CustomerBorrowed.
        let key = &keys[i];
        let age = &ages[i];
        let num_purchase = &num_purchases[i];
        let total_purchase = &total_purchases[i];
        let duration_spent = &duration_spents[i];
        let duration_since = &duration_sinces[i];
        let zip_code = &zip_codes[i];
        let address = &addresses[i];
        let country = &countries[i];
        let state = &states[i];
        let first_name = &first_names[i];
        let last_name = &last_names[i];
        let province = &provinces[i];
        let comment = &comments[i];
        let order = &orders[i];
        let customer = CustomerBorrowed::new(key, age, num_purchase, total_purchase, duration_spent, duration_since, 
                        zip_code, address, country, state, first_name, last_name, province, comment, order);
        customers.push(customer);
    }
    let elapsed = start.elapsed().as_millis();
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
    let elapsed = start.elapsed().as_millis();
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