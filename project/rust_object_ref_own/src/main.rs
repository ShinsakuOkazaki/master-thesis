extern crate rand;
use std::iter;
use std::env;
use std::time::Instant;
use std::io::prelude::*;
use std::fs::OpenOptions;
use rand::{Rng, thread_rng, SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Alphanumeric;
use rand::distributions::{Distribution, Uniform};

fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let field: i32 = args[2].parse().unwrap();
    run(field, size);
}

fn run(field: i32, size: usize) {
    match field {
        1 => run_ex_zip_code(size),
        2 => run_ex_address(size),
        3 => run_ex_country(size),
        _ => println!("Wrong input!")
    }
}

fn run_ex_zip_code(size: usize) {
    let start = Instant::now();
    let addresses = get_string_vector(size);
    let zip_codes = get_string_vector(size);
    let countries = get_string_vector(size);
    let (elapsed_create, customers) = create_customer_vecor(size, &addresses, zip_codes, &countries);
    let (elapsed_access, count) = access_zip_code(& customers);
    let elapsed_total = start.elapsed().as_nanos();
    write_to_file(size, "own", elapsed_create, elapsed_access, elapsed_total, count);
}

fn run_ex_address(size: usize) {
    let start = Instant::now();
    let addresses = get_string_vector(size);
    let zip_codes = get_string_vector(size);
    let countries = get_string_vector(size);
    let (elapsed_create, customers) = create_customer_vecor(size, &addresses, zip_codes, &countries);
    let (elapsed_access, count) = access_address(& customers);
    let elapsed_total = start.elapsed().as_nanos();
    write_to_file(size, "reference", elapsed_create, elapsed_access, elapsed_total, count);
}

fn run_ex_country(size: usize) {
    let start = Instant::now();
    let addresses = get_string_vector(size);
    let zip_codes = get_string_vector(size);
    let countries = get_string_vector(size);
    let (elapsed_create, customers) = create_customer_vecor(size, &addresses, zip_codes, &countries);
    let (elapsed_access, count) = access_country(& customers);
    let elapsed_total = start.elapsed().as_nanos();
    write_to_file(size, "slice", elapsed_create, elapsed_access, elapsed_total, count);
}

fn access_zip_code(customers: &Vec<Customer>) -> (u128, u128) {
    let mut count: u128 = 0;
    let len = customers.len();
    let start = Instant::now();
    for i in 0..len {
        let bytes = customers[i].zip_code.as_bytes();
        let b_len = bytes.len();
        for j in 0..b_len {
            count += bytes[j] as u128;
        }
    }
    let elapsed = start.elapsed().as_nanos();
    (elapsed, count)
}

fn access_address(customers: &Vec<Customer>) -> (u128, u128) {
    let mut count: u128 = 0;
    let len = customers.len();
    let start = Instant::now();
    for i in 0..len {
        let bytes = customers[i].address.as_bytes();
        let b_len = bytes.len();
        for j in 0..b_len {
            count += bytes[j] as u128;
        }
    }
    let elapsed = start.elapsed().as_nanos();
    (elapsed, count)
}

fn access_country(customers: &Vec<Customer>) -> (u128, u128) {
    let mut count: u128 = 0;
    let len = customers.len();
    let start = Instant::now();
    for i in 0..len {
        let bytes = customers[i].country.as_bytes();
        let b_len = bytes.len();
        for j in 0..b_len {
            count += bytes[j] as u128;
        }
    }
    let elapsed = start.elapsed().as_nanos();
    (elapsed, count) 
}

fn get_string_vector(size: usize) -> Vec<String> {
    let mut strings = Vec::with_capacity(size);
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    for _ in 0..size {
        let string: String = iter::repeat(())
                            .map(|()| rng.sample(Alphanumeric))
                            .take(5)
                            .collect();
        strings.push(string);
    }
    strings
}

fn create_customer_vecor<'a>(size: usize, addresses: &'a Vec<String>, mut zip_codes: Vec<String>, countries: &'a Vec<String>) -> (u128, Vec<Customer<'a>>) {
    let start = Instant::now();
    let mut customers: Vec<Customer> = Vec::with_capacity(size);
    for i in 0..size {
        let total_order = get_integer();
        let weight_order = get_float();
        let zip_code = zip_codes.pop().unwrap();
        let address = &(addresses[i]);
        let country = &(countries[i][..]);
        let customer = Customer::new(total_order, weight_order, zip_code, address, country);
        customers.push(customer);
    }
    let elapsed = start.elapsed().as_nanos();
    (elapsed, customers)
}

pub struct Customer<'a> {
    total_order: i32,
    weight_order: f32,
    zip_code: String,
    address: &'a String,
    country: &'a str
}

impl Customer<'_>  {
    pub fn new<'a>(total_order: i32, weight_order: f32, zip_code: String, address: &'a String, country: &'a str) -> Customer<'a> {
        Customer {
            total_order: total_order,
            weight_order: weight_order,
            zip_code: zip_code,
            address: address,
            country: country
        }
    }
}

// fn get_string() -> String {
//     seed: &[_] = &[1, 2, 3, 4];
//     let rng: StdRng = SeedableRng::from_seed(seed);
//     let rand_string: String = rng.sample_iter(&Alphanumeric)
//                                  .take(5)
//                                  .collect();
//     rand_string
// }

fn get_integer() -> i32 {
    let dist = Uniform::from(3..7);
    let num: i32 = dist.sample(&mut thread_rng()) as i32;
    num
}

fn get_float() -> f32{
    let dist = Uniform::from(0..1);
    let num: f32 = dist.sample(&mut thread_rng()) as f32;
    num
}

fn write_to_file(size: usize, field: &str, elapsed_create: u128, elapsed_access: u128, elapsed_total: u128, count: u128) {
    let output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}#{:?}#{:?}\n", 
                         size, field, elapsed_create, elapsed_access, elapsed_total, count);
    println!("{}",output);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");
}
