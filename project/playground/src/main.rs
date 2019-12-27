extern crate rand;


use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

fn main() {
    
    let mut es = create_intVec(10);
    let mut vector = Vec::new();
    for e in es {
        vector.push(e);
    }
    let v = vector;
}

fn create_intVec(size: usize) -> std::vec::Vec<i32> {
    let mut elements = Vec::new();
    for e in 0..size {
        elements.push((e as i32));
    }
    return elements;
}

fn create_charArr() -> std::vec::Vec<[char;10]> {
    let mut elements = Vec::new();
    for _i in 0..10 {
        let char_arr = ['a', 'b', 'c', 'd', 'e', 
                        'f', 'g', 'h', 'i', 'j'];
        elements.push(char_arr);
    }
    return elements;
}


fn getVec() -> std::vec::Vec<Customer> {
    let mut elements = Vec::new();
    let mut rng = thread_rng();
    for _i in 0..10 {
        let total_order = rng.gen::<i32>();
        let weight_order = rng.gen::<f32>();
        let zip_code : String = thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(10)
                                .collect();
        let customer = Customer::initialize(total_order, weight_order, zip_code);
        elements.push(customer);
    }
    return elements;
}

pub struct Customer {
    total_order: i32,
    weight_order: f32,
    zip_code: String
}

impl Customer {
    pub fn initialize(total_order: i32, weight_order: f32, zip_code: String) -> Customer {
        Customer {
            total_order: total_order,
            weight_order: weight_order,
            zip_code: zip_code
        }
    } 
}