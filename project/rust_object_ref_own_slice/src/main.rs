extern crate rand;
extern crate bytes;
use bytes::{Bytes, BytesMut, Buf, BufMut};
use std::iter;
use std::env;
use std::time::Instant;
use std::io::prelude::*;
use std::fs::OpenOptions;
use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Alphanumeric;
use std::io::Result;

fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let field: i32 = args[2].parse().unwrap();
    run(field, size);
}

fn run(field: i32, size: usize) {
    match field {
        1 => run_ex_owned(size),
        2 => run_ex_borrowed(size),
        3 => run_ex_slice(size),
        _ => println!("Wrong input!")
    }
}

fn run_ex_owned(size: usize) {
    let start = Instant::now();
    // Create String vectors.
    let addresses = get_string_vector(size);
    let zip_codes = get_string_vector(size);
    let countries = get_string_vector(size);
    // Create vector of CustomerOwned objects and take creation time.
    let (elapsed_create, customers) = create_customer_onwed_vector(size, addresses, zip_codes, countries);
    // Access to every feild of each object in the vector and take access time.
    let (elapsed_access, count) = access_owned(& customers).unwrap();
    let elapsed_total = start.elapsed().as_micros();
    write_to_file(size, "own", elapsed_create, elapsed_access, elapsed_total, count);
}

fn run_ex_borrowed(size: usize) {
    let start = Instant::now();
    // Create String vectors.
    let addresses = get_string_vector(size);
    let zip_codes = get_string_vector(size);
    let countries = get_string_vector(size);
    // Create vector of CustomerBorrowed objects and take creation time.
    let (elapsed_create, customers) = create_customer_borrowed_vector(size, &addresses, &zip_codes, &countries);
    // Access to every feild of each object in the vector and take access time.
    let (elapsed_access, count) = access_borrowed(& customers).unwrap();
    let elapsed_total = start.elapsed().as_micros();
    write_to_file(size, "reference", elapsed_create, elapsed_access, elapsed_total, count);
}

// Function to run experiment for object whose fields are slice.
fn run_ex_slice(size: usize) {
    let start = Instant::now();
    // Create String vectors.
    let addresses = get_string_vector(size);
    let zip_codes = get_string_vector(size);
    let countries = get_string_vector(size);
    // Create vector of CustomerSlice objects and take creation time.
    let (elapsed_create, customers) = create_customer_slice_vector(size, &addresses, &zip_codes, &countries);
    // Access to every feild of each object in the vector and take access time.   
    let (elapsed_access, count) = access_slice(& customers).unwrap();
    let elapsed_total = start.elapsed().as_micros();
    write_to_file(size, "slice", elapsed_create, elapsed_access, elapsed_total, count);
}


// Function access object whose field is owned.
fn access_owned(customers: &Vec<CustomerOwned>) -> Result<(u128, u128)>  {
    let len = customers.len();
    let start = Instant::now();
    let mut count: u128 = 0;
    for i in 0..len {
        // Serialize object to bytes and read the bytes back to new object.
        let before_customer = &customers[i];
        let bytes = write_byte_buffer(&before_customer);
        let buf = bytes.bytes();
        let after_customer = read_byte_buffer(buf)?;
        // To force to compile above code, access all of fields of newly created object by counting their length.
        count = (after_customer.zip_code.len() + after_customer.address.len() + after_customer.country.len()) as u128;
    }
    let elapsed = start.elapsed().as_micros(); 
    Ok((elapsed, count))
}

// Function access object whose field is borrowed.
fn access_borrowed(customers: &Vec<CustomerBorrowed>) -> Result<(u128, u128)> {
    let len = customers.len();
    let start = Instant::now();
    let mut count: u128 = 0;
    for i in 0..len {
        // Serialize object to bytes and read the bytes back to new object.
        let before_customer = &customers[i];
        let bytes = write_byte_buffer(&before_customer);
        let buf = bytes.bytes();
        let after_customer = read_byte_buffer(buf)?;
        // To force to compile above code, access all of fields of newly created object by counting their length.
        count = (after_customer.zip_code.len() + after_customer.address.len() + after_customer.country.len()) as u128;
    }
    let elapsed = start.elapsed().as_micros();
    Ok((elapsed, count))
}

// Function access object whose field is slice.
fn access_slice(customers: &Vec<CustomerSlice>) -> Result<(u128, u128)> {
    let len = customers.len();
    let start = Instant::now();
    let mut count: u128 = 0;
    for i in 0..len {
        let before_customer = &customers[i];
        let bytes = write_byte_buffer(&before_customer);
        let buf = bytes.bytes();
        let after_customer = read_byte_buffer(buf)?;
        count = (after_customer.zip_code.len() + after_customer.address.len() + after_customer.country.len()) as u128;
    }
    let elapsed = start.elapsed().as_micros();
    Ok((elapsed, count))
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

// Function to create a vector of CustomerOwned objects.
fn create_customer_onwed_vector(size: usize, mut addresses: Vec<String>, mut zip_codes: Vec<String>, mut countries: Vec<String>) -> (u128, Vec<CustomerOwned>) {
    let start = Instant::now();
    let mut customers: Vec<CustomerOwned> = Vec::with_capacity(size);
    for _ in 0..size {
        // Get owner by poping String from vector and create CustomerOwned.
        let zip_code = zip_codes.pop().unwrap();
        let address = addresses.pop().unwrap();
        let country = countries.pop().unwrap();
        let customer = CustomerOwned::new(zip_code, address, country);
        customers.push(customer);
    }
    let elapsed = start.elapsed().as_micros();
    (elapsed, customers)
}

fn create_customer_borrowed_vector<'a>(size: usize, addresses: &'a Vec<String>, zip_codes: &'a Vec<String>, countries: &'a Vec<String>) -> (u128, Vec<CustomerBorrowed<'a>>) {
    let start = Instant::now();
    let mut customers: Vec<CustomerBorrowed> = Vec::with_capacity(size);
    for i in 0..size {
        // Get reference by acceesing String in vector and create CustomerBorrowed.
        let zip_code = &zip_codes[i];
        let address = &addresses[i];
        let country = &countries[i];
        let customer = CustomerBorrowed::new(zip_code, address, country);
        customers.push(customer);
    }
    let elapsed = start.elapsed().as_micros();
    (elapsed, customers)
}

// Function to create Customer Vector 
fn create_customer_slice_vector<'a>(size: usize, addresses: &'a Vec<String>, zip_codes: &'a Vec<String>, countries: &'a Vec<String>) -> (u128, Vec<CustomerSlice<'a>>) {
    let start = Instant::now();
    let mut customers: Vec<CustomerSlice> = Vec::with_capacity(size);
    for i in 0..size {
        // Get slice by acceesing String in vector and create CustomerSlice.
        let zip_code = &(zip_codes[i][..]);
        let address = &(addresses[i][..]);
        let country = &(countries[i][..]);
        let customer = CustomerSlice::new(zip_code, address, country);
        customers.push(customer);
    }
    let elapsed = start.elapsed().as_micros();
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
    zip_code: String,
    address: String,
    country: String
}

// Objects whose fields are all borrowed.
pub struct CustomerBorrowed<'a> {
    zip_code: &'a String,
    address: &'a String,
    country: &'a String
}

// Objects whose fields are all slice.
pub struct CustomerSlice<'a> {
    zip_code: &'a str,
    address: &'a str,
    country: &'a str
}

// Implement new (constructor)
impl CustomerOwned  {
    pub fn new(zip_code: String, address: String, country: String) -> CustomerOwned {
        CustomerOwned {
            zip_code: zip_code,
            address: address,
            country: country
        }
    }
}
// Implement new (constructor)
impl CustomerBorrowed<'_> {
    pub fn new<'a>(zip_code: &'a String, address: &'a String, country: &'a String) -> CustomerBorrowed<'a> {
        CustomerBorrowed {
            zip_code: zip_code,
            address: address,
            country: country
        }
    }
}
// Implement new (constructor)
impl CustomerSlice<'_>{
    pub fn new<'a>(zip_code: &'a str, address: &'a str, country: &'a str) -> CustomerSlice<'a>{
        CustomerSlice {
            zip_code: zip_code,
            address: address,
            country: country
        }
    }
}

//Implement Trait to  Struct 
impl<'a> Customer for &'a CustomerOwned {
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
impl<'a> Customer for &'a CustomerBorrowed<'_>{
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
impl<'a> Customer for &'a CustomerSlice<'_> {
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

///
/// 
///
///  

// Function to write byte from Customer object
pub fn write_byte_buffer<T: Customer>(customer: &T) -> Bytes{
    let zip_code_buf = customer.zip_code_bytes(); 
    let address_buf = customer.address_bytes();
    let country_buf = customer.country_bytes();
    
    let mut byte_buffer = BytesMut::with_capacity(12 + zip_code_buf.len() + address_buf.len() + country_buf.len());

    byte_buffer.put_i32(zip_code_buf.len() as i32); // 4 bytes
    byte_buffer.put_slice(zip_code_buf);    

    byte_buffer.put_i32(address_buf.len() as i32); // 4 bytes
    byte_buffer.put_slice(address_buf);
    
    byte_buffer.put_i32(country_buf.len() as i32); // 4 bytes
    byte_buffer.put_slice(country_buf);

    return byte_buffer.freeze(); // Return immutable Bytes object.
}

// Function to read bytes to Customer Object
fn read_byte_buffer(buf: &[u8]) -> Result<CustomerOwned> {
    // 
    let mut byte_buffer_init = BytesMut::with_capacity(buf.len());
    byte_buffer_init.extend_from_slice(buf);
    let mut byte_buffer = byte_buffer_init.freeze();

    let mut string_size;

    string_size = byte_buffer.get_i32();
    let temp_zip_code = extract_string(&mut byte_buffer, string_size as usize)?;

    string_size = byte_buffer.get_i32();
    let temp_address = extract_string(&mut byte_buffer, string_size as usize)?;

    string_size = byte_buffer.get_i32();
    let temp_country = extract_string(&mut byte_buffer, string_size as usize)?;


    let customer = CustomerOwned::new(temp_zip_code, temp_address, temp_country);

    Ok(customer)
}

// Function to copy subset of bytes to bytes vector from Bytes object.
fn extract_string(byte_buffer: &mut Bytes, size: usize) -> Result<String> {
    let mut dst: Vec<u8> = vec![0; size];
    byte_buffer.copy_to_slice(&mut dst[..]);
    let string = String::from_utf8(dst).unwrap();
    Ok(string)
}
