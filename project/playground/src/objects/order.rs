use std::rc::Rc;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::cmp::Ordering;

#[derive(Clone, Debug)]
pub struct OrderOwned {
    order_id: i32,
    num_items: i32, 
    payment: f64,
    order_time: f64,
    title: String,
    comment: String
}
#[derive(Clone, Debug)]
pub struct OrderBorrowed<'a> {
    order_id: &'a i32,
    num_items: &'a i32, 
    payment: &'a f64,
    order_time: &'a f64,
    title: &'a String,
    comment: &'a String
}

#[derive(Clone, Debug)]
pub struct OrderRc {
    order_id: Rc<i32>,
    num_items: Rc<i32>, 
    payment: Rc<f64>,
    order_time: Rc<f64>,
    title: Rc<String>,
    comment: Rc<String>
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

impl OrderRc {
    pub fn new(order_id: Rc<i32>, num_items: Rc<i32>, payment: Rc<f64>, order_time: Rc<f64>, title: Rc<String>, comment: Rc<String>) -> OrderRc {
        OrderRc {
            order_id: order_id, 
            num_items: num_items, 
            payment: payment,
            order_time: order_time,
            title: title,
            comment: comment
        }
    }
}

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

impl Eq for OrderBorrowed<'_> {}

impl Ord for OrderBorrowed<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order_id.cmp(&other.order_id)
    }
}

impl PartialOrd for OrderBorrowed<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OrderBorrowed<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.order_id == other.order_id
    }
}

impl Eq for OrderRc {}

impl Ord for OrderRc {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order_id.cmp(&other.order_id)
    }
}

impl PartialOrd for OrderRc {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for OrderRc {
    fn eq(&self, other: &Self) -> bool {
        self.order_id == other.order_id
    }
}

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

pub fn get_order_borrowed_vector<'a>(size: usize, order_ids: &'a Vec<i32>, num_itemss: &'a Vec<i32>, 
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

pub fn get_order_rc_vector<'a>(size: usize, order_ids: &'a Vec<Rc<i32>>, num_itemss: &'a Vec<Rc<i32>>, 
    payments: &'a Vec<Rc<f64>>, order_times: &'a Vec<Rc<f64>>, titles: &'a Vec<Rc<String>>, comments: &'a Vec<Rc<String>>) -> Vec<Rc<OrderRc>> {
    let mut orders = Vec::with_capacity(size);
    for i in 0..size {
        let order_id = Rc::clone(&order_ids[i]);
        let num_items = Rc::clone(&num_itemss[i]);
        let payment = Rc::clone(&payments[i]);
        let order_time = Rc::clone(&order_times[i]);
        let title = Rc::clone(&titles[i]);
        let comment = Rc::clone(&comments[i]);
        let order = OrderRc::new(order_id, num_items, payment, order_time, title, comment);
        orders.push(Rc::new(order));
    }
    orders
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

impl Serialize for OrderRc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("OrderRc", 6)?;
        state.serialize_field("order_id", &self.order_id)?;
        state.serialize_field("num_items", &self.num_items)?;
        state.serialize_field("payment", &self.payment)?;
        state.serialize_field("order_time", &self.order_time)?;
        state.serialize_field("title", &self.title)?;
        state.serialize_field("comment", &self.comment)?;
        state.end()
    }
}