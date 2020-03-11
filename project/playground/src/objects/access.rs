use crate::objects::customer::*;
use serde::ser::Serialize;
use serde::de::DeserializeOwned;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::time::Instant;
use std::net::SocketAddr;
use std::error::Error;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::sync::Arc;


fn serialize<T>(customer: &T) 
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

fn serialize_vector<T, P>(customers: &[T], path: P) 
    where T: Customer + Serialize,
          P: AsRef<Path> 
{
    let serialized = serde_json::to_string(customers).unwrap();
    let mut file = OpenOptions::new()
                    .append(false)
                    .create(true)
                    .open(path)
                    .unwrap();
    file.write_all(serialized.as_bytes()).expect("Fail to write file.");
}

pub fn deserialize_vector<T, P>(path: P) -> Result<Vec<T>, Box<Error>>
    where T: Customer + Serialize + DeserializeOwned,
          P: AsRef<Path>
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let customers = serde_json::from_reader(reader)?;
    Ok(customers)
}


pub fn deserialize_vector_arc<T, P>(path: P) -> Result<Vec<Arc<T>>, Box<Error>>
    where T: Customer + Serialize + DeserializeOwned,
          P: AsRef<Path>
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let customers = serde_json::from_reader(reader)?;
    Ok(customers)
}

// Function access object whose field is owned.

pub fn access_owned(customers: &Vec<CustomerOwned>) -> u128 {
    let len = customers.len();
    let start = Instant::now();
    for i in 0..len {
        serialize(&customers[i])
    }
    let elapsed = start.elapsed().as_millis(); 
    elapsed
}

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
