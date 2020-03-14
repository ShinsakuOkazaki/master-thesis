//use std::rc::Rc;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};
use std::cmp::Ordering;
//use std::marker::Send;
//use std::sync::Arc;
use std::default::Default;
use std::fmt;

#[derive(Clone, Debug, Default)]
pub struct OrderOwned {
    order_id: i32,
    num_items: i32, 
    payment: f64,
    order_time: f64,
    title: String,
    comment: String
}
// #[derive(Clone, Debug)]
// pub struct OrderBorrowed<'a> {
//     order_id: &'a i32,
//     num_items: &'a i32, 
//     payment: &'a f64,
//     order_time: &'a f64,
//     title: &'a String,
//     comment: &'a String
// }

// #[derive(Clone, Debug, Default)]
// pub struct OrderRc {
//     order_id: Rc<i32>,
//     num_items: Rc<i32>, 
//     payment: Rc<f64>,
//     order_time: Rc<f64>,
//     title: Rc<String>,
//     comment: Rc<String>
// }
// #[derive(Clone, Debug, Default)]
// pub struct OrderArc {
//     order_id: Arc<i32>,
//     num_items: Arc<i32>, 
//     payment: Arc<f64>,
//     order_time: Arc<f64>,
//     title: Arc<String>,
//     comment: Arc<String>
// }
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

// impl OrderBorrowed<'_> {
//     pub fn new<'a>(order_id: &'a i32, num_items: &'a i32, payment: &'a f64, order_time: &'a f64, title: &'a String, comment: &'a String) -> OrderBorrowed<'a> {
//         OrderBorrowed {
//             order_id: order_id, 
//             num_items: num_items, 
//             payment: payment,
//             order_time: order_time,
//             title: title,
//             comment: comment
//         }
//     }
// }

// impl OrderRc {
//     pub fn new(order_id: Rc<i32>, num_items: Rc<i32>, payment: Rc<f64>, order_time: Rc<f64>, title: Rc<String>, comment: Rc<String>) -> OrderRc {
//         OrderRc {
//             order_id: order_id, 
//             num_items: num_items, 
//             payment: payment,
//             order_time: order_time,
//             title: title,
//             comment: comment
//         }
//     }
// }


// impl OrderArc {
//     pub fn new(order_id: Arc<i32>, num_items: Arc<i32>, payment: Arc<f64>, order_time: Arc<f64>, title: Arc<String>, comment: Arc<String>) -> OrderArc {
//         OrderArc {
//             order_id: order_id, 
//             num_items: num_items, 
//             payment: payment,
//             order_time: order_time,
//             title: title,
//             comment: comment
//         }
//     }
// }

impl Eq for OrderOwned {}

impl Ord for OrderOwned {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order_id.cmp(&other.order_id)
    }
}

impl PartialOrd for OrderOwned {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OrderOwned {
    fn eq(&self, other: &Self) -> bool {
        self.order_id == other.order_id
    }
}

// impl Eq for OrderBorrowed<'_> {}

// impl Ord for OrderBorrowed<'_> {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.order_id.cmp(&other.order_id)
//     }
// }

// impl PartialOrd for OrderBorrowed<'_> {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for OrderBorrowed<'_> {
//     fn eq(&self, other: &Self) -> bool {
//         self.order_id == other.order_id
//     }
// }

// impl Eq for OrderRc {}

// impl Ord for OrderRc {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.order_id.cmp(&other.order_id)
//     }
// }

// impl PartialOrd for OrderRc {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for OrderRc {
//     fn eq(&self, other: &Self) -> bool {
//         self.order_id == other.order_id
//     }
// }


// impl Eq for OrderArc {}

// impl Ord for OrderArc {
//     fn cmp(&self, other: &Self) -> Ordering {
//         self.order_id.cmp(&other.order_id)
//     }
// }

// impl PartialOrd for OrderArc {
//     fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
//         Some(self.cmp(other))
//     }
// }

// impl PartialEq for OrderArc {
//     fn eq(&self, other: &Self) -> bool {
//         self.order_id == other.order_id
//     }
// }


pub fn get_order_owned_vector(size: usize, mut order_ids: Vec<i32>, mut num_itemss: Vec<i32>, 
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

// pub fn get_order_borrowed_vector<'a>(size: usize, order_ids: &'a Vec<i32>, num_itemss: &'a Vec<i32>, 
//             payments: &'a Vec<f64>, order_times: &'a Vec<f64>, titles: &'a Vec<String>, comments: &'a Vec<String>) -> Vec<OrderBorrowed<'a>> {
//     let mut orders = Vec::with_capacity(size);
//     for i in 0..size {
//         let order_id = &order_ids[i];
//         let num_items = &num_itemss[i];
//         let payment = &payments[i];
//         let order_time = &order_times[i];
//         let title = &titles[i];
//         let comment = &comments[i];
//         let order = OrderBorrowed::new(order_id, num_items, payment, order_time, title, comment);
//         orders.push(order);
//     }
//     orders
// }

// pub fn get_order_rc_vector<'a>(size: usize, order_ids: &'a Vec<Rc<i32>>, num_itemss: &'a Vec<Rc<i32>>, 
//     payments: &'a Vec<Rc<f64>>, order_times: &'a Vec<Rc<f64>>, titles: &'a Vec<Rc<String>>, comments: &'a Vec<Rc<String>>) -> Vec<Rc<OrderRc>> {
//     let mut orders = Vec::with_capacity(size);
//     for i in 0..size {
//         let order_id = Rc::clone(&order_ids[i]);
//         let num_items = Rc::clone(&num_itemss[i]);
//         let payment = Rc::clone(&payments[i]);
//         let order_time = Rc::clone(&order_times[i]);
//         let title = Rc::clone(&titles[i]);
//         let comment = Rc::clone(&comments[i]);
//         let order = OrderRc::new(order_id, num_items, payment, order_time, title, comment);
//         orders.push(Rc::new(order));
//     }
//     orders
// }

// pub fn get_order_arc_vector<'a>(size: usize, order_ids: &'a Vec<Arc<i32>>, num_itemss: &'a Vec<Arc<i32>>, 
//     payments: &'a Vec<Arc<f64>>, order_times: &'a Vec<Arc<f64>>, titles: &'a Vec<Arc<String>>, comments: &'a Vec<Arc<String>>) -> Vec<Arc<OrderArc>> {
//     let mut orders = Vec::with_capacity(size);
//     for i in 0..size {
//         let order_id = Arc::clone(&order_ids[i]);
//         let num_items = Arc::clone(&num_itemss[i]);
//         let payment = Arc::clone(&payments[i]);
//         let order_time = Arc::clone(&order_times[i]);
//         let title = Arc::clone(&titles[i]);
//         let comment = Arc::clone(&comments[i]);
//         let order = OrderArc::new(order_id, num_items, payment, order_time, title, comment);
//         orders.push(Arc::new(order));
//     }
//     orders
// }


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

impl<'de> Deserialize<'de> for OrderOwned {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>,
    {
        enum Field {OrderId, NumItems, Payment, OrderTime, Title, Comment};

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where D: Deserializer<'de>, 
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("6 fields")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where E: de::Error, 
                    {
                        match value {
                            "order_id" => Ok(Field::OrderId), 
                            "num_items" => Ok(Field::NumItems), 
                            "payment" => Ok(Field::Payment),  
                            "order_time" => Ok(Field::OrderTime),
                            "title" => Ok(Field::Title),
                            "comment" => Ok(Field::Comment),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct OrderOwnedVisitor;

        impl<'de> Visitor<'de> for OrderOwnedVisitor {
            type Value = OrderOwned;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct OrderOwned")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<OrderOwned, V::Error>
            where V: SeqAccess<'de>,
            {
                let order_id = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                
                let num_items = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                
                let payment = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                
                let order_time = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let title = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                let comment = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                
                Ok(OrderOwned::new(order_id, num_items, payment, order_time, title, comment))
            }

            fn visit_map<V>(self, mut map: V) -> Result<OrderOwned, V::Error>
            where V: MapAccess<'de>,
            {
                let mut order_id = None;
                let mut num_items = None;
                let mut payment = None;
                let mut order_time = None;
                let mut title = None;
                let mut comment = None;
                

                while let Some(k) = map.next_key()? {
                    match k {
                        Field::OrderId => {
                            if  order_id.is_some() {
                                return Err(de::Error::duplicate_field("order_id"));
                            }
                            order_id = Some(map.next_value()?);
                        }
                        Field::NumItems => {
                            if num_items.is_some() {
                                return Err(de::Error::duplicate_field("num_items"));
                            }
                            num_items = Some(map.next_value()?);
                        }
                        Field::Payment => {
                            if payment.is_some() {
                                return Err(de::Error::duplicate_field("payment"));
                            }
                            payment = Some(map.next_value()?);
                        }
                        Field::OrderTime => {
                            if order_time.is_some() {
                                return Err(de::Error::duplicate_field("order_time"));
                            }
                            order_time = Some(map.next_value()?);
                        }
                        Field::Title => {
                            if title.is_some() {
                                return Err(de::Error::duplicate_field("title"));
                            }
                            title = Some(map.next_value()?);
                        }
                        Field::Comment => {
                            if comment.is_some() {
                                return Err(de::Error::duplicate_field("comment"));
                            }
                            comment = Some(map.next_value()?);
                        }
                    }
                }
                let order_id = order_id.ok_or_else(|| de::Error::missing_field("order_id"))?;
                let num_items = num_items.ok_or_else(|| de::Error::missing_field("num_items"))?;
                let payment = payment.ok_or_else(|| de::Error::missing_field("payment"))?;
                let order_time = order_time.ok_or_else(|| de::Error::missing_field("order_time"))?;
                let title = title.ok_or_else(|| de::Error::missing_field("title"))?;
                let comment = comment.ok_or_else(|| de::Error::missing_field("comment"))?;
                
                Ok(OrderOwned::new(order_id, num_items, payment, order_time, title, comment))
            }
        }
        const FIELDS: &'static [&'static str] = &["order_id", "num_items", "payment", "order_time", "title", "comment" ];
        deserializer.deserialize_struct("OrderOwned", FIELDS, OrderOwnedVisitor)
    }
}


// impl Serialize for OrderBorrowed<'_> {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("OrderBorrowed", 6)?;
//         state.serialize_field("order_id", self.order_id)?;
//         state.serialize_field("num_items", self.num_items)?;
//         state.serialize_field("payment", self.payment)?;
//         state.serialize_field("order_time", &self.order_time)?;
//         state.serialize_field("title", self.title)?;
//         state.serialize_field("comment", self.comment)?;
//         state.end()
//     }
// }

// impl Serialize for OrderRc {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("OrderRc", 6)?;
//         state.serialize_field("order_id", &self.order_id)?;
//         state.serialize_field("num_items", &self.num_items)?;
//         state.serialize_field("payment", &self.payment)?;
//         state.serialize_field("order_time", &self.order_time)?;
//         state.serialize_field("title", &self.title)?;
//         state.serialize_field("comment", &self.comment)?;
//         state.end()
//     }
// }


// impl Serialize for OrderArc {
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where S: Serializer,
//     {
//         let mut state = serializer.serialize_struct("OrderArc", 6)?;
//         state.serialize_field("order_id", &self.order_id)?;
//         state.serialize_field("num_items", &self.num_items)?;
//         state.serialize_field("payment", &self.payment)?;
//         state.serialize_field("order_time", &self.order_time)?;
//         state.serialize_field("title", &self.title)?;
//         state.serialize_field("comment", &self.comment)?;
//         state.end()
//     }
// }