extern crate rand;
extern crate bytes;
use std::iter;
use std::env;
use std::time::Instant;
use std::io::prelude::*;
use std::fs::OpenOptions;
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Alphanumeric;
use rand::distributions::{Uniform, Distribution};
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::collections::HashMap;

fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let init: bool = args[2].parse().unwrap();
    let field: i32 = args[3].parse().unwrap();
    run(field, init ,size);
}

fn run(field: i32, init: bool, size: usize) {
    match field {
        1 => run_ex_owned(init, size),
        2 => run_ex_borrowed(init, size),
        3 => run_ex_slice(init, size),
        _ => println!("Wrong input!")
    }
}

fn run_ex_owned(init: bool, size: usize) {
    warming_up(size);
    let start = Instant::now();
    let order_ids = get_integer_vector(size);
    let num_itemss = get_integer_vector(size);
    let payments = get_float_vector(size);
    let order_times = get_float_vector(size);
    let titles = get_string_vector(size);
    let comments_order = get_string_vector(size);
    let orders = get_order_owned_vector(size, order_ids, num_itemss, payments, order_times, titles, comments_order);
    
    let keys = get_integer_vector(size);
    let ages = get_integer_vector(size);
    let num_purchases = get_integer_vector(size);
    let total_purchases = get_float_vector(size);
    let duration_spents = get_float_vector(size);
    let duration_sinces = get_float_vector(size);
    let zip_codes = get_string_vector(size);
    let addresss = get_string_vector(size);
    let countrys = get_string_vector(size);
    let states = get_string_vector(size);
    let first_names = get_string_vector(size);
    let last_names = get_string_vector(size);
    let provinces = get_string_vector(size);
    let comments = get_string_vector(size);

    // Create vector of CustomerOwned objects and take creation time.
    let (elapsed_create, customers) = create_customer_onwed_vector(init, size, keys, ages, num_purchases, total_purchases, duration_spents, duration_sinces, 
                                                                    zip_codes, addresss, countrys, states, first_names, last_names, provinces, comments, orders);
    // Access to every feild of each object in the vector and take access time.
    let elapsed_access = access_owned(& customers);
    let elapsed_total = start.elapsed().as_millis();
    write_to_file(init, size, "own", elapsed_create, elapsed_access, elapsed_total);
}


fn run_ex_borrowed(init: bool, size: usize) {
    warming_up(size);
    let start = Instant::now();
    let order_ids = get_integer_vector(size);
    let num_itemss = get_integer_vector(size);
    let payments = get_float_vector(size);
    let order_times = get_float_vector(size);
    let titles = get_string_vector(size);
    let comments_order = get_string_vector(size);
    let orders = get_order_borrowed_vector(size, &order_ids, &num_itemss, &payments, &order_times, &titles, &comments_order);
    
    let keys = get_integer_vector(size);
    let ages = get_integer_vector(size);
    let num_purchases = get_integer_vector(size);
    let total_purchases = get_float_vector(size);
    let duration_spents = get_float_vector(size);
    let duration_sinces = get_float_vector(size);
    let zip_codes = get_string_vector(size);
    let addresss = get_string_vector(size);
    let countrys = get_string_vector(size);
    let states = get_string_vector(size);
    let first_names = get_string_vector(size);
    let last_names = get_string_vector(size);
    let provinces = get_string_vector(size);
    let comments = get_string_vector(size);
    
    // Create vector of CustomerBorrowed objects and take creation time.
    let (elapsed_create, customers) = create_customer_borrowed_vector(init, size, &keys, &ages, &num_purchases, &total_purchases, &duration_spents, &duration_sinces, 
                                                                &zip_codes, &addresss, &countrys, &states, &first_names, &last_names, &provinces, &comments, &orders);
    // Access to every feild of each object in the vector and take access time.
    let elapsed_access = access_borrowed(& customers);
    let elapsed_total = start.elapsed().as_millis();
    write_to_file(init, size, "reference", elapsed_create, elapsed_access, elapsed_total);
}

// Function to run experiment for object whose fields are slice.
fn run_ex_slice(init: bool, size: usize) {
    warming_up(size);
    let start = Instant::now();
    // Create String vectors.
    let order_ids = get_integer_vector(size);
    let num_itemss = get_integer_vector(size);
    let payments = get_float_vector(size);
    let order_times = get_float_vector(size);
    let titles = get_string_vector(size);
    let comments_order = get_string_vector(size);
    let orders = get_order_slice_vector(size, &order_ids, &num_itemss, &payments, &order_times, &titles, &comments_order);
    
    let keys = get_integer_vector(size);
    let ages = get_integer_vector(size);
    let num_purchases = get_integer_vector(size);
    let total_purchases = get_float_vector(size);
    let duration_spents = get_float_vector(size);
    let duration_sinces = get_float_vector(size);
    let zip_codes = get_string_vector(size);
    let addresss = get_string_vector(size);
    let countrys = get_string_vector(size);
    let states = get_string_vector(size);
    let first_names = get_string_vector(size);
    let last_names = get_string_vector(size);
    let provinces = get_string_vector(size);
    let comments = get_string_vector(size);
    // Create vector of CustomerSlice objects and take creation time.
    let (elapsed_create, customers) = create_customer_slice_vector(init, size, &keys, &ages, &num_purchases, &total_purchases, &duration_spents, &duration_sinces, 
        &zip_codes, &addresss, &countrys, &states, &first_names, &last_names, &provinces, &comments, &orders);
    // Access to every feild of each object in the vector and take access time.   
    let elapsed_access = access_slice(& customers);
    let elapsed_total = start.elapsed().as_millis();
    write_to_file(init, size, "slice", elapsed_create, elapsed_access, elapsed_total);
}

fn warming_up(size: usize) {
    let mut count_integer = HashMap::new();
    let mut count_string = HashMap::new();

    let mut count;

    let mut order_ids;
    let mut num_itemss;
    let mut titles;
    let mut comments_order;
    for _ in 0..2 {
        order_ids = get_integer_vector(size);
        num_itemss = get_integer_vector(size);
        titles = get_string_vector(size);
        comments_order = get_string_vector(size);
        for _ in 0..size {
            let order_id = order_ids.pop().unwrap();
            if count_integer.contains_key(&order_id) {
                count = count_integer.get(&order_id).unwrap() + 1;
                count_integer.insert(order_id, count);
            } else {
                count_integer.insert(order_id, 0);
            }
            let num_items = num_itemss.pop().unwrap();
            if count_integer.contains_key(&num_items) {
                count = count_integer.get(&num_items).unwrap() + 1;
                count_integer.insert(num_items, count);
            } else {
                count_integer.insert(num_items, 0);
            }
            let title = titles.pop().unwrap();
            if count_string.contains_key(&title) {
                count = count_string.get(&title).unwrap() + 1;
                count_string.insert(title, count);
            } else {
                count_string.insert(title, 0);
            }
            let comment_order = comments_order.pop().unwrap();
            if count_string.contains_key(&comment_order) {
                count = count_string.get(&comment_order).unwrap() + 1;
                count_string.insert(comment_order, count);
            } else {
                count_string.insert(comment_order, 0);
            }
        }
    }

    let mut keys;
    let mut ages;
    let mut num_purchases;
    let mut zip_codes;
    let mut addresss;
    let mut countrys;
    let mut states;
    let mut first_names;
    let mut last_names;
    let mut provinces;
    let mut comments;
    for _ in 0..2 {
        keys = get_integer_vector(size);
        ages = get_integer_vector(size);
        num_purchases = get_integer_vector(size);
        zip_codes = get_string_vector(size);
        addresss = get_string_vector(size);
        countrys = get_string_vector(size);
        states = get_string_vector(size);
        first_names = get_string_vector(size);
        last_names = get_string_vector(size);
        provinces = get_string_vector(size);
        comments = get_string_vector(size);
        for _ in 0..size {
            let key = keys.pop().unwrap();
            if count_integer.contains_key(&key) {
                count = count_integer.get(&key).unwrap() + 1;
                count_integer.insert(key, count);
            } else {
                count_integer.insert(key, 0);
            }
            let age = ages.pop().unwrap();
            if count_integer.contains_key(&age) {
                count = count_integer.get(&age).unwrap() + 1;
                count_integer.insert(age, count);
            } else {
                count_integer.insert(age, 0);
            }
            let num_purchase = num_purchases.pop().unwrap();
            if count_integer.contains_key(&num_purchase) {
                count = count_integer.get(&num_purchase).unwrap() + 1;
                count_integer.insert(num_purchase, count);
            } else {
                count_integer.insert(num_purchase, 0);
            }
            let zip_code = zip_codes.pop().unwrap();
            if count_string.contains_key(&zip_code) {
                count = count_string.get(&zip_code).unwrap() + 1;
                count_string.insert(zip_code, count);
            } else {
                count_string.insert(zip_code, 0);
            }
            let address = addresss.pop().unwrap();
            if count_string.contains_key(&address) {
                count = count_string.get(&address).unwrap() + 1;
                count_string.insert(address, count);
            } else {
                count_string.insert(address, 0);
            }
            let country = countrys.pop().unwrap();
            if count_string.contains_key(&country) {
                count = count_string.get(&country).unwrap() + 1;
                count_string.insert(country, count);
            } else {
                count_string.insert(country, 0);
            }
            let state = states.pop().unwrap();
            if count_string.contains_key(&state) {
                count = count_string.get(&state).unwrap() + 1;
                count_string.insert(state, count);
            } else {
                count_string.insert(state, 0);
            }
            let first_name = first_names.pop().unwrap();
            if count_string.contains_key(&first_name) {
                count = count_string.get(&first_name).unwrap() + 1;
                count_string.insert(first_name, count);
            } else {
                count_string.insert(first_name, 0);
            }
            let last_name = last_names.pop().unwrap();
            if count_string.contains_key(&last_name) {
                count = count_string.get(&last_name).unwrap() + 1;
                count_string.insert(last_name, count);
            } else {
                count_string.insert(last_name, 0);
            }
            let province = provinces.pop().unwrap();
            if count_string.contains_key(&province) {
                count = count_string.get(&province).unwrap() + 1;
                count_string.insert(province, count);
            } else {
                count_string.insert(province, 0);
            }
            let comment = comments.pop().unwrap();
            if count_string.contains_key(&comment) {
                count = count_string.get(&comment).unwrap() + 1;
                count_string.insert(comment, count);
            } else {
                count_string.insert(comment, 0);
            }
        }
    }
    println!("Warmed Up!! created HashMap with {} and {}", count_integer.len(), count_string.len());
}

fn de_serialize<T>(customer: &T) 
    where T: Customer + Serialize, 
{
    let serialized = serde_json::to_string(&customer).unwrap();
    let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open("string.log")
            .unwrap();
    file.write_all(serialized.as_bytes()).expect("Fail to write file.");
}

// Function access object whose field is owned.
fn access_owned(customers: &Vec<CustomerOwned>) -> u128 {
    let len = customers.len();
    let start = Instant::now();
    for i in 0..len {
        de_serialize(&customers[i])
    }
    let elapsed = start.elapsed().as_millis(); 
    elapsed
}

// Function access object whose field is borrowed.
fn access_borrowed(customers: &Vec<CustomerBorrowed>) -> u128 {
    let len = customers.len();
    let start = Instant::now();
    for i in 0..len {
        de_serialize(&customers[i])
    }
    let elapsed = start.elapsed().as_millis(); 
    elapsed
}

// Function access object whose field is slice.
fn access_slice(customers: &Vec<CustomerSlice>) -> u128 {
    let len = customers.len();
    let start = Instant::now();
    for i in 0..len {
        de_serialize(&customers[i])
    }
    let elapsed = start.elapsed().as_millis(); 
    elapsed
}


// Function to get string vector.
// All vector will have the same elements with length of 5.
fn get_string_vector(size: usize) -> Vec<String> {
    let mut strings = Vec::with_capacity(size);
    // Set random seed.
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    for _ in 0..size {
        // Get random Stirng whose length is 5.
        let string: String = iter::repeat(())
                            .map(|()| rng.sample(Alphanumeric))
                            .take(5)
                            .collect();
        strings.push(string);
    }
    strings
}

fn get_integer_vector(size: usize) -> Vec<i32> {
    let mut integers = Vec::with_capacity(size);
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    let dist = Uniform::from(1..5);
    let mut num: i32;
    for _ in 0..size {
        num = dist.sample(&mut rng) as i32;
        integers.push(num);
    }
    integers    
}

fn get_float_vector(size: usize) -> Vec<f64> {
    let mut floats = Vec::with_capacity(size);
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    let dist = Uniform::from(0..1);
    let mut num: f64;
    for _ in 0..size {
        num = dist.sample(&mut rng) as f64;
        floats.push(num);
    }
    floats
}

fn get_order_owned_vector(size: usize, mut order_ids: Vec<i32>, mut num_itemss: Vec<i32>, 
                        mut payments: Vec<f64>, mut order_times: Vec<f64>, mut titles: Vec<String>, mut comments: Vec<String>) -> Vec<OrderOwned> {
    let mut orders = Vec::with_capacity(size);
    for _ in 0..size {
        let order_id = order_ids.pop().unwrap();
        let num_items = num_itemss.pop().unwrap();
        let payment = payments.pop().unwrap();
        let order_time = order_times.pop().unwrap();
        let title = titles.pop().unwrap();
        let comment = comments.pop().unwrap();
        let order = OrderOwned::new(order_id, num_items, payment, order_time, title, comment);
        orders.push(order);
    }
    orders
}

fn get_order_borrowed_vector<'a>(size: usize, order_ids: &'a Vec<i32>, num_itemss: &'a Vec<i32>, 
                                payments: &'a Vec<f64>, order_times: &'a Vec<f64>, titles: &'a Vec<String>, comments: &'a Vec<String>) -> Vec<OrderBorrowed<'a>> {
    let mut orders = Vec::with_capacity(size);
    for i in 0..size {
        let order_id = &order_ids[i];
        let num_items = &num_itemss[i];
        let payment = &payments[i];
        let order_time = &order_times[i];
        let title = &titles[i];
        let comment = &comments[i];
        let order = OrderBorrowed::new(order_id, num_items, payment, order_time, title, comment);
        orders.push(order);
    }
    orders
}


fn get_order_slice_vector<'a>(size: usize, order_ids: &'a Vec<i32>, num_itemss: &'a Vec<i32>, 
                        payments: &'a Vec<f64>, order_times: &'a Vec<f64>, titles: &'a Vec<String>, comments: &'a Vec<String>) -> Vec<OrderSlice<'a>> {

    let mut orders = Vec::with_capacity(size);
    for i in 0..size {
        let order_id = &order_ids[i];
        let num_items = &num_itemss[i];
        let payment = &payments[i];
        let order_time = &order_times[i];
        let title = &(titles[i][..]);
        let comment = &(comments[i][..]);
        let order = OrderSlice::new(order_id, num_items, payment, order_time, title, comment);
        orders.push(order);
    }
    orders
}



// Function to create a vector of CustomerOwned objects.
fn create_customer_onwed_vector(init: bool, size: usize, mut keys: Vec<i32>, mut ages: Vec<i32>, mut num_purchases: Vec<i32>, 
                                mut total_purchases: Vec<f64>, mut duration_spents: Vec<f64>, mut duration_sinces: Vec<f64>,
                                mut zip_codes: Vec<String>, mut addresses: Vec<String> ,mut countries: Vec<String>,
                                mut states: Vec<String>, mut first_names: Vec<String>, mut last_names: Vec<String>,
                                mut provinces: Vec<String>, mut comments: Vec<String>, mut orders: Vec<OrderOwned>) -> (u128, Vec<CustomerOwned>) {
    let start = Instant::now();
    let mut customers: Vec<CustomerOwned>;
    if init {
        customers = Vec::with_capacity(size);
    } else {
        customers = Vec::new();
    }
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


fn create_customer_borrowed_vector<'a>(init: bool, size: usize, keys: &'a Vec<i32>, ages: &'a Vec<i32>, num_purchases: &'a Vec<i32>, 
                                        total_purchases: &'a Vec<f64>, duration_spents: &'a Vec<f64>, duration_sinces: &'a Vec<f64>,
                                        zip_codes: &'a Vec<String>, addresses: &'a Vec<String> , countries: &'a Vec<String>,
                                        states: &'a Vec<String>, first_names: &'a Vec<String>, last_names: &'a Vec<String>,
                                        provinces: &'a Vec<String>, comments: &'a Vec<String>, orders: &'a Vec<OrderBorrowed>) -> (u128, Vec<CustomerBorrowed<'a>>) {
    let start = Instant::now();
    let mut customers: Vec<CustomerBorrowed>;
    if init {
        customers = Vec::with_capacity(size);
    } else {
        customers = Vec::new();
    }
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

// Function to create Customer Vector 
fn create_customer_slice_vector<'a>(init: bool, size: usize, keys: &'a Vec<i32>, ages: &'a Vec<i32>, num_purchases: &'a Vec<i32>, 
                                    total_purchases: &'a Vec<f64>, duration_spents: &'a Vec<f64>, duration_sinces: &'a Vec<f64>,
                                    zip_codes: &'a Vec<String>, addresses: &'a Vec<String> , countries: &'a Vec<String>,
                                    states: &'a Vec<String>, first_names: &'a Vec<String>, last_names: &'a Vec<String>,
                                    provinces: &'a Vec<String>, comments: &'a Vec<String>, orders: &'a Vec<OrderSlice>) -> (u128, Vec<CustomerSlice<'a>>) {
    let start = Instant::now();
    let mut customers: Vec<CustomerSlice>;
    if init {
        customers = Vec::with_capacity(size);
    } else {
        customers = Vec::new();
    }

    for i in 0..size {
        // Get slice by acceesing String in vector and create CustomerSlice.
        let key = &keys[i];
        let age = &ages[i];
        let num_purchase = &num_purchases[i];
        let total_purchase = &total_purchases[i];
        let duration_spent = &duration_spents[i];
        let duration_since = &duration_sinces[i];
        let zip_code = &(zip_codes[i][..]);
        let address = &(addresses[i][..]);
        let country = &(countries[i][..]);
        let state = &(states[i][..]);
        let first_name = &(first_names[i][..]);
        let last_name = &(last_names[i][..]);
        let province = &(provinces[i][..]);
        let comment = &(comments[i][..]);
        let order = &orders[i];
        let customer = CustomerSlice::new(key, age, num_purchase, total_purchase, duration_spent, duration_since, zip_code, address, country, state, first_name, last_name, province, comment, order);
        customers.push(customer);
    }
    let elapsed = start.elapsed().as_millis();
    (elapsed, customers)
}

// Custorm trait (interface for all objects.)
pub trait Customer {
    fn zip_code_bytes(&self) -> &[u8];
    fn address_bytes(&self)  -> &[u8];
    fn country_bytes(&self)  -> &[u8];
}



// Objects whose fields are all owned.
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

// Objects whose fields are all slice.
pub struct CustomerSlice<'a> {
    key: &'a i32,
    age: &'a i32,
    num_purchase: &'a i32,
    total_purchase: &'a f64,
    duration_spent: &'a f64, 
    duration_since: &'a f64,
    zip_code: &'a str,
    address: &'a str,
    country: &'a str, 
    state: &'a str,
    first_name: &'a str,
    last_name: &'a str,
    province: &'a str,
    comment: &'a str,
    order: &'a OrderSlice<'a>
}


pub struct OrderOwned {
    order_id: i32,
    num_items: i32, 
    payment: f64,
    order_time: f64,
    title: String,
    comment: String
}


pub struct OrderBorrowed<'a> {
    order_id: &'a i32,
    num_items: &'a i32, 
    payment: &'a f64,
    order_time: &'a f64,
    title: &'a String,
    comment: &'a String
}

pub struct OrderSlice<'a> {
    order_id: &'a i32,
    num_items: &'a i32, 
    payment: &'a f64,
    order_time: &'a f64,
    title: &'a str,
    comment: &'a str
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
// Implement new (constructor)
impl CustomerSlice<'_>{
    pub fn new<'a>(key: &'a i32, age: &'a i32, num_purchase: &'a i32, total_purchase: &'a f64, duration_spent: &'a f64, duration_since: &'a f64, 
                    zip_code: &'a str, address: &'a str, country: &'a str, state: &'a str, 
                    first_name: &'a str, last_name: &'a str, province: &'a str, comment: &'a str, order: &'a OrderSlice) -> CustomerSlice<'a>{
        CustomerSlice {
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

impl OrderOwned {
    pub fn new(order_id: i32, num_items: i32, payment: f64, order_time: f64, title: String, comment: String) -> OrderOwned {
        OrderOwned {
            order_id: order_id, 
            num_items: num_items, 
            payment: payment,
            order_time: order_time,
            title: title,
            comment: comment
        }
    }
}

impl OrderBorrowed<'_> {
    pub fn new<'a>(order_id: &'a i32, num_items: &'a i32, payment: &'a f64, order_time: &'a f64, title: &'a String, comment: &'a String) -> OrderBorrowed<'a> {
        OrderBorrowed {
            order_id: order_id, 
            num_items: num_items, 
            payment: payment,
            order_time: order_time,
            title: title,
            comment: comment
        }
    }
}

impl OrderSlice<'_> {
    pub fn new<'a>(order_id: &'a i32, num_items: &'a i32, payment: &'a f64, order_time: &'a f64, title: &'a str, comment: &'a str) -> OrderSlice<'a> {
        OrderSlice {
            order_id: order_id, 
            num_items: num_items, 
            payment: payment,
            order_time: order_time,
            title: title,
            comment: comment
        }
    }
}

impl Serialize for OrderOwned {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("OrderOwned", 6)?;
        state.serialize_field("order_id", &self.order_id)?;
        state.serialize_field("num_items", &self.num_items)?;
        state.serialize_field("payment", &self.payment)?;
        state.serialize_field("order_time", &self.order_time)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("comment", &self.comment)?;
        state.end()
    }
}


impl Serialize for OrderBorrowed<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("OrderBorrowed", 6)?;
        state.serialize_field("order_id", self.order_id)?;
        state.serialize_field("num_items", self.num_items)?;
        state.serialize_field("payment", self.payment)?;
        state.serialize_field("order_time", &self.order_time)?;
        state.serialize_field("title", self.title)?;
        state.serialize_field("comment", self.comment)?;
        state.end()
    }
}

impl Serialize for OrderSlice<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("OrderSlice", 6)?;
        state.serialize_field("order_id", self.order_id)?;
        state.serialize_field("num_items", self.num_items)?;
        state.serialize_field("payment", self.payment)?;
        state.serialize_field("order_time", self.order_time)?;
        state.serialize_field("title", self.title)?;
        state.serialize_field("comment", self.comment)?;
        state.end()
    }
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


impl Serialize for CustomerSlice<'_> {
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
impl Customer for CustomerSlice<'_> {
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




// Function to write result to file.
fn write_to_file(init: bool, size: usize, field: &str, elapsed_create: u128, elapsed_access: u128, elapsed_total: u128) {
    let output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}#{:?}#{:?}\n", 
                         init, size, field, elapsed_create, elapsed_access, elapsed_total);
    println!("{}",output);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");
}
