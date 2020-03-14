use crate::objects::customer::*;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::error::Error;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::sync::Arc;
use std::collections::HashMap;
use std::time::Instant;


fn serialize<T>(customer: &T) 
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

fn serialize_arc<T>(customer: Arc<T>) 
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





pub fn serialize_vector<T>(customers: &[T], partition: &String) 
    where T: Customer + Serialize
{
    let serialized = serde_json::to_string(&customers).unwrap();
    let path = Path::new(&partition);
    let mut file = OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&path)
                    .unwrap();
    file.write_all(serialized.as_bytes()).expect("Fail to write file.");
}

pub fn serialize_vector_arc<T>(customers: Vec<Arc<T>>, partition: &String) 
where T: Customer + Serialize
{
    let serialized = serde_json::to_string(&customers).unwrap();
    let path = Path::new(&partition);
    let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .open(&path)
                .unwrap();
    file.write_all(serialized.as_bytes()).expect("Fail to write file.");
} 

pub fn deserialize_vector<T, P>(path: P) -> Result<Vec<T>, Box<dyn Error>>
    where T: Customer + Serialize + DeserializeOwned,
          P: AsRef<Path>
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let customers = serde_json::from_reader(reader)?;
    Ok(customers)
}


pub fn deserialize_vector_arc<T, P>(path: P) -> Result<Vec<Arc<T>>, Box<dyn Error>>
    where T: Customer + Serialize + DeserializeOwned,
          P: AsRef<Path>
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let customers = serde_json::from_reader(reader)?;
    Ok(customers)
}



pub fn access_aggregate(aggregate: &HashMap<String, Vec<CustomerOwned>>) -> u128 {
    let start = Instant::now();
    for (_k, v) in aggregate {
        for i in 0..v.len() {
            serialize(&v[i]);
        } 
    }
    let elapsed = start.elapsed().as_millis();
    elapsed
}

pub fn access_aggregate_arc(aggregate: &HashMap<String, Vec<Arc<CustomerOwned>>>) -> u128 {
    let start = Instant::now();
    for (_k, v) in aggregate {
        for i in 0..v.len() {
            serialize_arc(Arc::clone(&v[i]));
        } 
    }
    let elapsed = start.elapsed().as_millis();
    elapsed
}


//Function access object whose field is owned.

// pub fn access_owned(customers: &Vec<CustomerOwned>) -> u128 {
//     let len = customers.len();
//     let start = Instant::now();
//     for i in 0..len {
//         serialize(&customers[i])
//     }
//     let elapsed = start.elapsed().as_millis(); 
//     elapsed
// }

// // Function access object whose field is borrowed.
// pub fn access_borrowed(customers: &Vec<CustomerBorrowed>) -> u128 {
//     let len = customers.len();
//     let start = Instant::now();
//     for i in 0..len {
//         serialize(&customers[i])
//     }
//     let elapsed = start.elapsed().as_millis(); 
//     elapsed
// }



// pub fn access_rc(customers: &Vec<CustomerRc>) -> u128 {
//     let len = customers.len();
//     let start = Instant::now();
//     for i in 0..len {
//         serialize(&customers[i])
//     }
//     let elapsed = start.elapsed().as_millis(); 
//     elapsed
// }
