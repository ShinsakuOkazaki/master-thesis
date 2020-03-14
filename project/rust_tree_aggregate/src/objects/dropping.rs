use std::time::Instant;
use crate::objects::customer::*;

pub fn drop_owned(customers: &mut Vec<CustomerOwned>) -> u128  {
    let len = customers.len();
    let start = Instant::now();
    for _ in 0..len {
        let customer = customers.pop();
        drop(customer);
    }
    let elapsed = start.elapsed().as_millis(); 
    elapsed
}

pub fn drop_borrowed(customers: &mut Vec<CustomerBorrowed>) -> u128  {
    let len = customers.len();
    let start = Instant::now();
    for _ in 0..len {
        let customer = customers.pop();
        drop(customer);
    }
    let elapsed = start.elapsed().as_millis(); 
    elapsed
}

pub fn drop_rc(customers: &mut Vec<CustomerRc>) -> u128 {
    let len = customers.len();
    let start = Instant::now();
    for _ in 0..len {
        let customer = customers.pop();
        drop(customer);
    }
    let elapsed = start.elapsed().as_millis(); 
    elapsed
}
