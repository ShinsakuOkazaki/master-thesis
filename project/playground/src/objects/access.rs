use crate::objects::customer::*;
use serde::ser::Serialize;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::Instant;

fn de_serialize<T>(customer: &T) 
    where T: Customer + Serialize, 
{
    let serialized = serde_json::to_string(&customer).unwrap();
    let mut file = OpenOptions::new()
            .append(true)
            .create(true)
            .open("string.log")
            .unwrap();
    file.write_all(serialized.as_bytes()).expect("Fail to write file.");
}

// Function access object whose field is owned.

pub fn access_owned(customers: &Vec<CustomerOwned>) -> u128 {
    let len = customers.len();
    let start = Instant::now();
    for i in 0..len {
        de_serialize(&customers[i])
    }
    let elapsed = start.elapsed().as_millis(); 
    elapsed
}

// Function access object whose field is borrowed.
pub fn access_borrowed(customers: &Vec<CustomerBorrowed>) -> u128 {
    let len = customers.len();
    let start = Instant::now();
    for i in 0..len {
        de_serialize(&customers[i])
    }
    let elapsed = start.elapsed().as_millis(); 
    elapsed
}



pub fn access_rc(customers: &Vec<CustomerRc>) -> u128 {
    let len = customers.len();
    let start = Instant::now();
    for i in 0..len {
        de_serialize(&customers[i])
    }
    let elapsed = start.elapsed().as_millis(); 
    elapsed
}
