use crate::objects::order::OrderOwned;
//use std::rc::Rc;
use std::time::Instant;
use std::cmp::Ordering;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use std::fmt;
use std::marker::{Send, Sync};
//use std::sync::Arc;
use std::default::Default;

// Custorm trait (interface for all objects.)
pub trait Customer {
    fn zip_code_bytes(&self) -> &[u8];
    fn address_bytes(&self)  -> &[u8];
    fn country_bytes(&self)  -> &[u8];
}



// Objects whose fields are all owned.
#[derive(Clone, Debug, Default)]
pub struct CustomerOwned {
    pub key: i32,
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
// #[derive(Clone, Debug)]
// pub struct CustomerBorrowed<'a> {
//     key: &'a i32,
//     age: &'a i32,
//     num_purchase: &'a i32,
//     total_purchase: &'a f64,
//     duration_spent: &'a f64, 
//     duration_since: &'a f64,
//     zip_code: &'a String,
//     address: &'a String,
//     country: &'a String,
//     state: &'a String,
//     first_name: &'a String,
//     last_name: &'a String,
//     province: &'a String,
//     comment: &'a String, 
//     order: &'a OrderBorrowed<'a>
// }
// #[derive(Clone, Debug, Default)]
// pub struct CustomerRc {
//     key: Rc<i32>,
//     age: Rc<i32>,
//     num_purchase: Rc<i32>,
//     total_purchase: Rc<f64>,
//     duration_spent: Rc<f64>, 
//     duration_since: Rc<f64>,
//     zip_code: Rc<String>,
//     address: Rc<String>,
//     country: Rc<String>,
//     state: Rc<String>,
//     first_name: Rc<String>,
//     last_name: Rc<String>,
//     province: Rc<String>,
//     comment: Rc<String>, 
//     order: Rc<OrderRc>
// }


// #[derive(Clone, Debug, Default)]
// pub struct CustomerArc {
//     key: Arc<i32>,
//     age: Arc<i32>,
//     num_purchase: Arc<i32>,
//     total_purchase: Arc<f64>,
//     duration_spent: Arc<f64>, 
//     duration_since: Arc<f64>,
//     zip_code: Arc<String>,
//     address: Arc<String>,
//     country: Arc<String>,
//     state: Arc<String>,
//     first_name: Arc<String>,
//     last_name: Arc<String>,
//     province: Arc<String>,
//     comment: Arc<String>, 
//     order: Arc<OrderArc>
// }


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
// impl CustomerBorrowed<'_> {
//     pub fn new<'a>(key: &'a i32, age: &'a i32, num_purchase: &'a i32, total_purchase: &'a f64, duration_spent: &'a f64, duration_since: &'a f64, 
//                     zip_code: &'a String, address: &'a String, country: &'a String, state: &'a String, 
//                     first_name: &'a String, last_name: &'a String, province: &'a String, comment: &'a String, order: &'a OrderBorrowed) -> CustomerBorrowed<'a> {
//         CustomerBorrowed {
//             key: key, 
//             age: age,
//             num_purchase: num_purchase, 
//             total_purchase: total_purchase,
//             duration_spent: duration_spent,
//             duration_since: duration_since,
//             zip_code: zip_code,
//             address: address,
//             country: country,
//             state: state,
//             first_name: first_name, 
//             last_name: last_name, 
//             province: province,
//             comment: comment,
//             order: order
//         }
//     }
// }


// impl CustomerRc {
//     pub fn new(key: Rc<i32>, age: Rc<i32>, num_purchase: Rc<i32>, total_purchase: Rc<f64>, duration_spent: Rc<f64>, duration_since: Rc<f64>, 
//         zip_code: Rc<String>, address: Rc<String>, country: Rc<String>, state: Rc<String>, 
//         first_name: Rc<String>, last_name: Rc<String>, province: Rc<String>, comment: Rc<String>, order: Rc<OrderRc>) -> CustomerRc{
//         CustomerRc {
//             key: key, 
//             age: age,
//             num_purchase: num_purchase, 
//             total_purchase: total_purchase,
//             duration_spent: duration_spent,
//             duration_since: duration_since,
//             zip_code: zip_code,
//             address: address,
//             country: country,
//             state: state,
//             first_name: first_name, 
//             last_name: last_name, 
//             province: province,
//             comment: comment,
//             order: order
//         }
//     }
// }

// impl CustomerArc {
//     pub fn new(key: Arc<i32>, age: Arc<i32>, num_purchase: Arc<i32>, total_purchase: Arc<f64>, duration_spent: Arc<f64>, duration_since: Arc<f64>, 
//         zip_code: Arc<String>, address: Arc<String>, country: Arc<String>, state: Arc<String>, 
//         first_name: Arc<String>, last_name: Arc<String>, province: Arc<String>, comment: Arc<String>, order: Arc<OrderArc>) -> CustomerArc{
//         CustomerArc {
//             key: key, 
//             age: age,
//             num_purchase: num_purchase, 
//             total_purchase: total_purchase,
//             duration_spent: duration_spent,
//             duration_since: duration_since,
//             zip_code: zip_code,
//             address: address,
//             country: country,
//             state: state,
//             first_name: first_name, 
//             last_name: last_name, 
//             province: province,
//             comment: comment,
//             order: order
//         }
//     }
// }




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
// impl Customer for CustomerBorrowed<'_>{
//     fn zip_code_bytes(&self) -> &[u8] {
//         self.zip_code.as_bytes()
//     }
//     fn address_bytes(&self) -> &[u8] {
//         self.address.as_bytes()
//     }
//     fn country_bytes(&self) -> &[u8] {
//         self.country.as_bytes()
//     }
// }

// // Implement Trait to  Struct 
// impl Customer for CustomerRc {
//     fn zip_code_bytes(&self) -> &[u8] {
//         self.zip_code.as_bytes()
//     }
//     fn address_bytes(&self) -> &[u8] {
//         self.address.as_bytes()
//     }
//     fn country_bytes(&self) -> &[u8] {
//         self.country.as_bytes()
//     }
// }


// impl Customer for CustomerArc {
//     fn zip_code_bytes(&self) -> &[u8] {
//         self.zip_code.as_bytes()
//     }
//     fn address_bytes(&self) -> &[u8] {
//         self.address.as_bytes()
//     }
//     fn country_bytes(&self) -> &[u8] {
//         self.country.as_bytes()
//     }
// }


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

// impl Eq for CustomerBorrowed<'_> {}

// impl Ord for CustomerBorrowed<'_> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.key.cmp(&other.key)
//     }
// }

// impl PartialOrd for CustomerBorrowed<'_> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for CustomerBorrowed<'_> {
//     fn eq(&self, other: &Self) -> bool {
//         self.key == other.key
//     }
// }

// impl Eq for CustomerRc {}

// impl Ord for CustomerRc {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.key.cmp(&other.key)
//     }
// }

// impl PartialOrd for CustomerRc {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for CustomerRc {
//     fn eq(&self, other: &Self) -> bool {
//         self.key == other.key
//     }
// }

// impl Eq for CustomerArc {}

// impl Ord for CustomerArc {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.key.cmp(&other.key)
//     }
// }

// impl PartialOrd for CustomerArc {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for CustomerArc {
//     fn eq(&self, other: &Self) -> bool {
//         self.key == other.key
//     }
// }


impl fmt::Display for CustomerOwned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "{}", self.key)
    }
}

// impl fmt::Display for CustomerBorrowed<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Use `self.number` to refer to each positional data point.
//         write!(f, "{}", self.key)
//     }
// }

// impl fmt::Display for CustomerRc {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Use `self.number` to refer to each positional data point.
//         write!(f, "{}", self.key)
//     }
// }

// impl fmt::Display for CustomerArc {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         // Use `self.number` to refer to each positional data point.
//         write!(f, "{}", self.key)
//     }
// }

unsafe impl Send for CustomerOwned {}
// unsafe impl Send for CustomerBorrowed<'_> {}
// unsafe impl Send for CustomerRc {}
// unsafe impl Send for CustomerArc {}


unsafe impl Sync for CustomerOwned {}
// unsafe impl Sync for CustomerBorrowed<'_> {}
// unsafe impl Sync for CustomerRc {}
// unsafe impl Sync for CustomerArc {}


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

// pub fn create_customer_borrowed_vector<'a>(size: usize, keys: &'a Vec<i32>, ages: &'a Vec<i32>, num_purchases: &'a Vec<i32>, 
//                 total_purchases: &'a Vec<f64>, duration_spents: &'a Vec<f64>, duration_sinces: &'a Vec<f64>,
//                 zip_codes: &'a Vec<String>, addresses: &'a Vec<String> , countries: &'a Vec<String>,
//                 states: &'a Vec<String>, first_names: &'a Vec<String>, last_names: &'a Vec<String>,
//                 provinces: &'a Vec<String>, comments: &'a Vec<String>, orders: &'a Vec<OrderBorrowed>) -> (u128, Vec<CustomerBorrowed<'a>>) {
//     let start = Instant::now();
//     let mut customers: Vec<CustomerBorrowed> = Vec::new();
//     for i in 0..size {
//         // Get reference by acceesing String in vector and create CustomerBorrowed.
//         let key = &keys[i];
//         let age = &ages[i];
//         let num_purchase = &num_purchases[i];
//         let total_purchase = &total_purchases[i];
//         let duration_spent = &duration_spents[i];
//         let duration_since = &duration_sinces[i];
//         let zip_code = &zip_codes[i];
//         let address = &addresses[i];
//         let country = &countries[i];
//         let state = &states[i];
//         let first_name = &first_names[i];
//         let last_name = &last_names[i];
//         let province = &provinces[i];
//         let comment = &comments[i];
//         let order = &orders[i];
//         let customer = CustomerBorrowed::new(key, age, num_purchase, total_purchase, duration_spent, duration_since, 
//                         zip_code, address, country, state, first_name, last_name, province, comment, order);
//         customers.push(customer);
//     }
//     let elapsed = start.elapsed().as_millis();
//     (elapsed, customers)
// }

// // Function to create a vector of CustomerOwned objects.
// pub fn create_customer_rc_vector<'a>(size: usize, keys: &'a Vec<Rc<i32>>, ages: &'a Vec<Rc<i32>>, num_purchases: &'a Vec<Rc<i32>>, 
//     total_purchases: &'a Vec<Rc<f64>>, duration_spents: &'a Vec<Rc<f64>>, duration_sinces: &'a Vec<Rc<f64>>,
//     zip_codes: &'a Vec<Rc<String>>, addresses: &'a Vec<Rc<String>> , countries: &'a Vec<Rc<String>>,
//     states: &'a Vec<Rc<String>>, first_names: &'a Vec<Rc<String>>, last_names: &'a Vec<Rc<String>>,
//     provinces: &'a Vec<Rc<String>>, comments: &'a Vec<Rc<String>>, orders: &'a Vec<Rc<OrderRc>>) -> (u128, Vec<CustomerRc>) {
//     let start = Instant::now();
//     let mut customers: Vec<CustomerRc> = Vec::new();
//     for i in 0..size {
//         // Get reference by acceesing String in vector and create CustomerBorrowed.
//         let key = Rc::clone(&keys[i]);
//         let age = Rc::clone(&ages[i]);
//         let num_purchase = Rc::clone(&num_purchases[i]);
//         let total_purchase = Rc::clone(&total_purchases[i]);
//         let duration_spent = Rc::clone(&duration_spents[i]);
//         let duration_since = Rc::clone(&duration_sinces[i]);
//         let zip_code = Rc::clone(&zip_codes[i]);
//         let address = Rc::clone(&addresses[i]);
//         let country = Rc::clone(&countries[i]);
//         let state = Rc::clone(&states[i]);
//         let first_name = Rc::clone(&first_names[i]);
//         let last_name = Rc::clone(&last_names[i]);
//         let province = Rc::clone(&provinces[i]);
//         let comment = Rc::clone(&comments[i]);
//         let order = Rc::clone(&orders[i]);
//         let customer = CustomerRc::new(key, age, num_purchase, total_purchase, duration_spent, duration_since, 
//         zip_code, address, country, state, first_name, last_name, province, comment, order);
//         customers.push(customer);
//     }
//     let elapsed = start.elapsed().as_millis();
//     (elapsed, customers)
// }

// pub fn create_customer_arc_vector<'a>(size: usize, keys: &'a Vec<Arc<i32>>, ages: &'a Vec<Arc<i32>>, num_purchases: &'a Vec<Arc<i32>>, 
//     total_purchases: &'a Vec<Arc<f64>>, duration_spents: &'a Vec<Arc<f64>>, duration_sinces: &'a Vec<Arc<f64>>,
//     zip_codes: &'a Vec<Arc<String>>, addresses: &'a Vec<Arc<String>> , countries: &'a Vec<Arc<String>>,
//     states: &'a Vec<Arc<String>>, first_names: &'a Vec<Arc<String>>, last_names: &'a Vec<Arc<String>>,
//     provinces: &'a Vec<Arc<String>>, comments: &'a Vec<Arc<String>>, orders: &'a Vec<Arc<OrderArc>>) -> (u128, Vec<CustomerArc>) {
//     let start = Instant::now();
//     let mut customers: Vec<CustomerArc> = Vec::new();
//     for i in 0..size {
//         // Get reference by acceesing String in vector and create CustomerBorrowed.
//         let key = Arc::clone(&keys[i]);
//         let age = Arc::clone(&ages[i]);
//         let num_purchase = Arc::clone(&num_purchases[i]);
//         let total_purchase = Arc::clone(&total_purchases[i]);
//         let duration_spent = Arc::clone(&duration_spents[i]);
//         let duration_since = Arc::clone(&duration_sinces[i]);
//         let zip_code = Arc::clone(&zip_codes[i]);
//         let address = Arc::clone(&addresses[i]);
//         let country = Arc::clone(&countries[i]);
//         let state = Arc::clone(&states[i]);
//         let first_name = Arc::clone(&first_names[i]);
//         let last_name = Arc::clone(&last_names[i]);
//         let province = Arc::clone(&provinces[i]);
//         let comment = Arc::clone(&comments[i]);
//         let order = Arc::clone(&orders[i]);
//         let customer = CustomerArc::new(key, age, num_purchase, total_purchase, duration_spent, duration_since, 
//         zip_code, address, country, state, first_name, last_name, province, comment, order);
//         customers.push(customer);
//     }
//     let elapsed = start.elapsed().as_millis();
//     (elapsed, customers)
// }


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


impl<'de> Deserialize<'de> for CustomerOwned {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>,
    {
        enum Field {Key, Age, NumPurchase, TotalPurchase, DurationSpent, DurationSince, 
                    ZipCode, Address, Country, State, FirstName, 
                    LastName, Province, Comment, Order};

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where D: Deserializer<'de>, 
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("9 fields")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where E: de::Error, 
                    {
                        match value {
                            "key" => Ok(Field::Key), 
                            "age" => Ok(Field::Age), 
                            "num_purchase" => Ok(Field::NumPurchase),  
                            "total_purchase" => Ok(Field::TotalPurchase),
                            "duration_spent" => Ok(Field::DurationSpent),
                            "duration_since" => Ok(Field::DurationSince),
                            "zip_code" => Ok(Field::ZipCode),
                            "address" => Ok(Field::Address),
                            "country" => Ok(Field::Country),
                            "state" => Ok(Field::State),
                            "first_name" => Ok(Field::FirstName),
                            "last_name" => Ok(Field::LastName),
                            "province" => Ok(Field::Province),
                            "comment" => Ok(Field::Comment),
                            "order" => Ok(Field::Order),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct CustomerOwnedVisitor;

        impl<'de> Visitor<'de> for CustomerOwnedVisitor {
            type Value = CustomerOwned;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct CostomerOwned")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<CustomerOwned, V::Error>
            where V: SeqAccess<'de>,
            {
                let key = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                
                let age = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                
                let num_purchase = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                
                let total_purchase = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let duration_spent = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let duration_since = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let zip_code = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let address = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let country = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let state = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let first_name = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let last_name = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let province = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let comment = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let order = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                
                Ok(CustomerOwned::new(
                    key, age, num_purchase, total_purchase, duration_spent, duration_since,
                    zip_code, address, country, state, first_name, last_name, province, comment, order))
            }

            fn visit_map<V>(self, mut map: V) -> Result<CustomerOwned, V::Error>
            where V: MapAccess<'de>,
            {
                let mut key = None;
                let mut age = None;
                let mut num_purchase = None;
                let mut total_purchase = None;
                let mut duration_spent = None;
                let mut duration_since = None;
                let mut zip_code = None;
                let mut address = None;
                let mut country = None;
                let mut state = None;
                let mut first_name = None;
                let mut last_name = None;
                let mut province = None;
                let mut comment = None;
                let mut order = None;

                while let Some(k) = map.next_key()? {
                    match k {
                        Field::Key => {
                            if key.is_some() {
                                return Err(de::Error::duplicate_field("key"));
                            }
                            key = Some(map.next_value()?);
                        }
                        Field::Age => {
                            if age.is_some() {
                                return Err(de::Error::duplicate_field("age"));
                            }
                            age = Some(map.next_value()?);
                        }
                        Field::NumPurchase => {
                            if num_purchase.is_some() {
                                return Err(de::Error::duplicate_field("num_purchase"));
                            }
                            num_purchase = Some(map.next_value()?);
                        }
                        Field::TotalPurchase => {
                            if total_purchase.is_some() {
                                return Err(de::Error::duplicate_field("total_purchase"));
                            }
                            total_purchase = Some(map.next_value()?);
                        }
                        Field::DurationSpent => {
                            if duration_spent.is_some() {
                                return Err(de::Error::duplicate_field("duration_spent"));
                            }
                            duration_spent = Some(map.next_value()?);
                        }
                        Field::DurationSince => {
                            if duration_since.is_some() {
                                return Err(de::Error::duplicate_field("duration_since"));
                            }
                            duration_since = Some(map.next_value()?);
                        }
                        Field::ZipCode => {
                            if zip_code.is_some() {
                                return Err(de::Error::duplicate_field("zip_code"));
                            }
                            zip_code = Some(map.next_value()?);
                        }
                        Field::Address => {
                            if address.is_some() {
                                return Err(de::Error::duplicate_field("address"));
                            }
                            address = Some(map.next_value()?);
                        }
                        Field::Country => {
                            if country.is_some() {
                                return Err(de::Error::duplicate_field("country"));
                            }
                            country = Some(map.next_value()?);
                        }
                        Field::State => {
                            if state.is_some() {
                                return Err(de::Error::duplicate_field("state"));
                            }
                            state = Some(map.next_value()?);
                        }
                        Field::FirstName => {
                            if first_name.is_some() {
                                return Err(de::Error::duplicate_field("first_name"));
                            }
                            first_name = Some(map.next_value()?);
                        }
                        Field::LastName => {
                            if last_name.is_some() {
                                return Err(de::Error::duplicate_field("last_name"));
                            }
                            last_name = Some(map.next_value()?);
                        }
                        Field::Province => {
                            if province.is_some() {
                                return Err(de::Error::duplicate_field("province"));
                            }
                            province = Some(map.next_value()?);
                        }
                        Field::Comment => {
                            if comment.is_some() {
                                return Err(de::Error::duplicate_field("comment"));
                            }
                            comment = Some(map.next_value()?);
                        }
                        Field::Order => {
                            if order.is_some() {
                                return Err(de::Error::duplicate_field("order"));
                            }
                            order = Some(map.next_value()?);
                        }
                    }
                }
                let key = key.ok_or_else(|| de::Error::missing_field("key"))?;
                let age = age.ok_or_else(|| de::Error::missing_field("age"))?;
                let num_purchase = num_purchase.ok_or_else(|| de::Error::missing_field("num_purchase"))?;
                let total_purchase = total_purchase.ok_or_else(|| de::Error::missing_field("total_purchase"))?;
                let duration_spent = duration_spent.ok_or_else(|| de::Error::missing_field("duration_spent"))?;
                let duration_since = duration_since.ok_or_else(|| de::Error::missing_field("duration_since"))?;
                let zip_code = zip_code.ok_or_else(|| de::Error::missing_field("zip_code"))?;
                let address = address.ok_or_else(|| de::Error::missing_field("address"))?;
                let country = country.ok_or_else(|| de::Error::missing_field("country"))?;
                let state = state.ok_or_else(|| de::Error::missing_field("state"))?;
                let first_name = first_name.ok_or_else(|| de::Error::missing_field("first_name"))?;
                let last_name = last_name.ok_or_else(|| de::Error::missing_field("last_name"))?;
                let province = province.ok_or_else(|| de::Error::missing_field("province"))?;
                let comment = comment.ok_or_else(|| de::Error::missing_field("comment"))?;
                let order = order.ok_or_else(|| de::Error::missing_field("order"))?;
                Ok(CustomerOwned::new(
                    key, age, num_purchase, total_purchase, duration_spent, duration_since,
                    zip_code, address, country, state, first_name, last_name, province, comment, order))
            }
        }
        const FIELDS: &'static [&'static str] = &["key", "age", "num_purchase", "total_purchase", "duration_spent", 
                                                  "duration_since", "zip_code", "address", "country", "state", 
                                                  "first_name", "last_name", "province", "comment", "order"];
        deserializer.deserialize_struct("CustomerOwned", FIELDS, CustomerOwnedVisitor)
    }
}



// impl Serialize for CustomerBorrowed<'_> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("CustomerBorrowed", 6).unwrap();
//         state.serialize_field("key", self.key)?;
//         state.serialize_field("age", self.age)?;
//         state.serialize_field("num_purchase", self.num_purchase)?;
//         state.serialize_field("total_purchase", self.total_purchase)?;
//         state.serialize_field("duration_spent", self.duration_spent)?;
//         state.serialize_field("duration_since", self.duration_since)?;
//         state.serialize_field("zip_code", self.zip_code)?;
//         state.serialize_field("address", self.address)?;
//         state.serialize_field("country", self.country)?;
//         state.serialize_field("state", self.state)?;
//         state.serialize_field("first_name", self.first_name)?;
//         state.serialize_field("last_name", self.last_name)?;
//         state.serialize_field("province", self.province)?;
//         state.serialize_field("comment", self.comment)?;
//         state.serialize_field("order", self.order)?;
//         state.end()
//     }
// }

// impl Serialize for CustomerRc {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("CustomerRc", 6).unwrap();
//         state.serialize_field("key", &self.key)?;
//         state.serialize_field("age", &self.age)?;
//         state.serialize_field("num_purchase", &self.num_purchase)?;
//         state.serialize_field("total_purchase", &self.total_purchase)?;
//         state.serialize_field("duration_spent", &self.duration_spent)?;
//         state.serialize_field("duration_since", &self.duration_since)?;
//         state.serialize_field("zip_code", &self.zip_code)?;
//         state.serialize_field("address", &self.address)?;
//         state.serialize_field("country", &self.country)?;
//         state.serialize_field("state", &self.state)?;
//         state.serialize_field("first_name", &self.first_name)?;
//         state.serialize_field("last_name", &self.last_name)?;
//         state.serialize_field("province", &self.province)?;
//         state.serialize_field("comment", &self.comment)?;
//         state.serialize_field("order", &self.order)?;
//         state.end()
//     }
// }


// impl Serialize for CustomerArc {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("CustomerArc", 6).unwrap();
//         state.serialize_field("key", &self.key)?;
//         state.serialize_field("age", &self.age)?;
//         state.serialize_field("num_purchase", &self.num_purchase)?;
//         state.serialize_field("total_purchase", &self.total_purchase)?;
//         state.serialize_field("duration_spent", &self.duration_spent)?;
//         state.serialize_field("duration_since", &self.duration_since)?;
//         state.serialize_field("zip_code", &self.zip_code)?;
//         state.serialize_field("address", &self.address)?;
//         state.serialize_field("country", &self.country)?;
//         state.serialize_field("state", &self.state)?;
//         state.serialize_field("first_name", &self.first_name)?;
//         state.serialize_field("last_name", &self.last_name)?;
//         state.serialize_field("province", &self.province)?;
//         state.serialize_field("comment", &self.comment)?;
//         state.serialize_field("order", &self.order)?;
//         state.end()
//     }
// }