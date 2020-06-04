use std::rc::Rc;
use std::time::Instant;
use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::path::Path;
use std::io::{BufRead, BufReader};
use std::fs::File;
use std::collections::HashMap;

pub trait LineItem{
    fn get_order_key(&mut self) -> i32;
    fn return_flag_byte(&self) -> &[u8];
    fn line_status_byte(&self) -> &[u8];
    fn ship_date_byte(&self) -> &[u8];
    fn commit_date_byte(&self) -> &[u8];
    fn receipt_date_byte(&self) -> &[u8];
    fn shipin_struct_byte(&self) -> &[u8];
    fn ship_mode_byte(&self) -> &[u8];
    fn comment_byte(&self) -> &[u8];
}



// Objects whose fields are all owned.
pub struct LineItemOwned{
    order_key: i32,
    part_key: i32, 
    suppkey: i32, 
    line_number: i32, 
    quantity: f64, 
    extended_price: f64,
    discount: f64,
    tax: f64, 
    return_flag: String,
    line_status: String,
    ship_date: String,
    commit_date: String,
    receipt_date: String,
    shipin_struct: String,
    ship_mode: String,
    comment: String,
}

// Objects whose fields are all borrowed.
pub struct LineItemBorrowed<'a>{
    order_key: &'a i32,
    part_key: &'a i32, 
    suppkey: &'a i32, 
    line_number: &'a i32, 
    quantity: &'a f64, 
    extended_price: &'a f64,
    discount: &'a f64,
    tax: &'a f64, 
    return_flag: &'a String,
    line_status: &'a String,
    ship_date: &'a String,
    commit_date: &'a String,
    receipt_date: &'a String,
    shipin_struct:&'a  String,
    ship_mode: &'a String,
    comment: &'a String,
}


pub struct LineItemRc{
    order_key: Rc<i32>,
    part_key: Rc<i32>, 
    suppkey: Rc<i32>, 
    line_number: Rc<i32>, 
    quantity: Rc<f64>, 
    extended_price: Rc<f64>,
    discount: Rc<f64>,
    tax: Rc<f64>, 
    return_flag: Rc<String>,
    line_status: Rc<String>,
    ship_date: Rc<String>,
    commit_date: Rc<String>,
    receipt_date: Rc<String>,
    shipin_struct: Rc<String>,
    ship_mode: Rc<String>,
    comment: Rc<String>,
}


impl LineItemOwned {
    pub fn new(order_key: i32, part_key: i32, suppkey: i32, line_number: i32, quantity: f64, extended_price: f64,
               discount: f64, tax: f64, return_flag: String, line_status: String, ship_date: String, commit_date: String,
               receipt_date: String, shipin_struct: String, ship_mode: String, comment: String) -> LineItemOwned {
        LineItemOwned {
            order_key: order_key,
            part_key: part_key, 
            suppkey: suppkey, 
            line_number: line_number, 
            quantity: quantity, 
            extended_price: extended_price,
            discount: discount,
            tax: tax, 
            return_flag: return_flag,
            line_status: line_status,
            ship_date: ship_date,
            commit_date: commit_date,
            receipt_date: receipt_date,
            shipin_struct: shipin_struct,
            ship_mode: ship_mode,
            comment: comment,
        }
    }
}


impl LineItemBorrowed<'_> {
    pub fn new<'a>(order_key: &'a i32, part_key: &'a i32, suppkey: &'a i32, line_number: &'a i32, quantity: &'a f64, extended_price: &'a f64,
                   discount: &'a f64, tax: &'a f64, return_flag: &'a String, line_status: &'a String, ship_date: &'a String, commit_date: &'a String,
                   receipt_date: &'a String, shipin_struct:&'a  String, ship_mode: &'a String, comment: &'a String) -> LineItemBorrowed<'a> {
        LineItemBorrowed {
            order_key: order_key,
            part_key: part_key, 
            suppkey: suppkey, 
            line_number: line_number, 
            quantity: quantity, 
            extended_price: extended_price,
            discount: discount,
            tax: tax, 
            return_flag: return_flag,
            line_status: line_status,
            ship_date: ship_date,
            commit_date: commit_date,
            receipt_date: receipt_date,
            shipin_struct: shipin_struct,
            ship_mode: ship_mode,
            comment: comment,
        }
    }
}



impl LineItemRc {
    pub fn new(order_key: Rc<i32>, part_key: Rc<i32>, suppkey: Rc<i32>, line_number: Rc<i32>, quantity: Rc<f64>, extended_price: Rc<f64>,
               discount: Rc<f64>, tax: Rc<f64>, return_flag: Rc<String>, line_status: Rc<String>, ship_date: Rc<String>, commit_date: Rc<String>,
               receipt_date: Rc<String>, shipin_struct: Rc<String>, ship_mode: Rc<String>, comment: Rc<String>) -> LineItemRc{
        LineItemRc{
            order_key: order_key,
            part_key: part_key, 
            suppkey: suppkey, 
            line_number: line_number, 
            quantity: quantity, 
            extended_price: extended_price,
            discount: discount,
            tax: tax, 
            return_flag: return_flag,
            line_status: line_status,
            ship_date: ship_date,
            commit_date: commit_date,
            receipt_date: receipt_date,
            shipin_struct: shipin_struct,
            ship_mode: ship_mode,
            comment: comment,
        }
    }
}

impl LineItem for LineItemOwned {

    fn get_order_key(&mut self) -> i32 {
        self.order_key
    }

    fn return_flag_byte(&self) -> &[u8] {
        self.return_flag.as_bytes()
    }
    fn line_status_byte(&self) -> &[u8]{
        self.line_status.as_bytes()
    }

    fn ship_date_byte(&self) -> &[u8]{
        self.ship_date.as_bytes()
    }

    fn commit_date_byte(&self) -> &[u8]{
        self.commit_date.as_bytes()
    }

    fn receipt_date_byte(&self) -> &[u8]{
        self.receipt_date.as_bytes()
    }

    fn shipin_struct_byte(&self) -> &[u8]{
        self.shipin_struct.as_bytes()
    }

    fn ship_mode_byte(&self) -> &[u8]{
        self.ship_mode.as_bytes()
    }

    fn comment_byte(&self) -> &[u8]{
        self.comment.as_bytes()
    }

}

impl LineItem for LineItemBorrowed<'_>{

    fn get_order_key(&mut self) -> i32 {
        let res = *self.order_key;
        res
    }

    fn return_flag_byte(&self) -> &[u8] {
        self.return_flag.as_bytes()
    }
    fn line_status_byte(&self) -> &[u8]{
        self.line_status.as_bytes()
    }

    fn ship_date_byte(&self) -> &[u8]{
        self.ship_date.as_bytes()
    }

    fn commit_date_byte(&self) -> &[u8]{
        self.commit_date.as_bytes()
    }

    fn receipt_date_byte(&self) -> &[u8]{
        self.receipt_date.as_bytes()
    }

    fn shipin_struct_byte(&self) -> &[u8]{
        self.shipin_struct.as_bytes()
    }

    fn ship_mode_byte(&self) -> &[u8]{
        self.ship_mode.as_bytes()
    }

    fn comment_byte(&self) -> &[u8]{
        self.comment.as_bytes()
    }

}


impl LineItem for LineItemRc { 
    
    fn get_order_key(&mut self) -> i32 {
        let res = *Rc::get_mut(&mut self.order_key).unwrap();
        res
    }
    fn return_flag_byte(&self) -> &[u8] {
        self.return_flag.as_bytes()
    }
    fn line_status_byte(&self) -> &[u8]{
        self.line_status.as_bytes()
    }

    fn ship_date_byte(&self) -> &[u8]{
        self.ship_date.as_bytes()
    }

    fn commit_date_byte(&self) -> &[u8]{
        self.commit_date.as_bytes()
    }

    fn receipt_date_byte(&self) -> &[u8]{
        self.receipt_date.as_bytes()
    }

    fn shipin_struct_byte(&self) -> &[u8]{
        self.shipin_struct.as_bytes()
    }

    fn ship_mode_byte(&self) -> &[u8]{
        self.ship_mode.as_bytes()
    }

    fn comment_byte(&self) -> &[u8]{
        self.comment.as_bytes()
    }

}

pub fn create_lineitem_onwed_vector(file_name: &str) -> (u128, Vec<LineItemOwned>) {
    let start = Instant::now();
    let path= Path::new(&file_name);
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let mut lineitems = Vec::new();
    for line in lines {
        let l = line.unwrap();
        let row: Vec<&str> = l.split('|').collect();
        let order_key: i32 = row[0].parse::<i32>().unwrap();
        let part_key: i32 = row[1].parse::<i32>().unwrap();
        let suppkey: i32 = row[2].parse::<i32>().unwrap();
        let line_number: i32 = row[3].parse::<i32>().unwrap(); 
        let quantity: f64 = row[4].parse::<f64>().unwrap();
        let extended_price: f64 = row[5].parse::<f64>().unwrap();
        let discount: f64 = row[6].parse::<f64>().unwrap();
        let tax: f64 = row[7].parse::<f64>().unwrap();
        let return_flag: String = row[8].to_string();
        let line_status: String = row[9].to_string();
        let ship_date: String = row[10].to_string();
        let commit_date: String = row[11].to_string();
        let receipt_date: String = row[12].to_string();
        let shipin_struct: String = row[13].to_string();
        let ship_mode: String = row[14].to_string();
        let comment: String = row[15].to_string();

        let lineitem = LineItemOwned::new(order_key, part_key, suppkey, line_number, quantity, extended_price, 
                                     discount, tax, return_flag, line_status, ship_date, commit_date, 
                                     receipt_date, shipin_struct, ship_mode, comment);
        
        lineitems.push(lineitem);
    }
    let elapsed = start.elapsed().as_micros();
    (elapsed, lineitems)
}

pub fn craete_lineitem_borrowed_vector<'a>(lineitems_owned: &'a [LineItemOwned]) -> (u128, Vec<LineItemBorrowed<'a>>) {
    let size = lineitems_owned.len();
    let start = Instant::now();
    let mut lineitems_borrowed = Vec::with_capacity(size);

    for i in 0..size {
        let lineitem_owend: &'a LineItemOwned = lineitems_owned.get(i).unwrap();
        let order_key = &lineitem_owend.order_key;
        let part_key = &&lineitem_owend.part_key;
        let suppkey = &lineitem_owend.suppkey;
        let line_number = &lineitem_owend.line_number;
        let quantity = &lineitem_owend.quantity;
        let extended_price = &lineitem_owend.extended_price;
        let discount = &lineitem_owend.discount;
        let tax = &lineitem_owend.tax;
        let return_flag = &lineitem_owend.return_flag;
        let line_status = &lineitem_owend.line_status;
        let ship_date = &lineitem_owend.ship_date;
        let commit_date = &lineitem_owend.commit_date;
        let receipt_date = &lineitem_owend.receipt_date;
        let shipin_struct = &lineitem_owend.shipin_struct;
        let ship_mode = &lineitem_owend.ship_mode;
        let comment = &lineitem_owend.comment;

        let lineitem_borrowed = LineItemBorrowed::new(order_key, part_key, suppkey, line_number, quantity, extended_price, 
                                     discount, tax, return_flag, line_status, ship_date, commit_date, 
                                     receipt_date, shipin_struct, ship_mode, comment);

        lineitems_borrowed.push(lineitem_borrowed);
    }
    let elapsed = start.elapsed().as_micros();
    (elapsed, lineitems_borrowed)
}


pub fn craete_lineitem_rc_vector(file_name: &str) -> (u128, Vec<LineItemRc>){
    let start = Instant::now();
    let path= Path::new(&file_name);
    let file = File::open(path).unwrap();
    let buf_reader = BufReader::new(file);
    let lines = buf_reader.lines();
    let mut lineitems_rc = Vec::new();
    for line in lines {
        let l = line.unwrap();
        let row: Vec<&str> = l.split('|').collect();
        let order_key: Rc<i32> = Rc::new(row[0].parse::<i32>().unwrap());
        let part_key: Rc<i32> = Rc::new(row[1].parse::<i32>().unwrap());
        let suppkey: Rc<i32> = Rc::new(row[2].parse::<i32>().unwrap());
        let line_number: Rc<i32> = Rc::new(row[3].parse::<i32>().unwrap()); 
        let quantity: Rc<f64> = Rc::new(row[4].parse::<f64>().unwrap());
        let extended_price: Rc<f64> = Rc::new(row[5].parse::<f64>().unwrap());
        let discount: Rc<f64> = Rc::new(row[6].parse::<f64>().unwrap());
        let tax: Rc<f64> = Rc::new(row[7].parse::<f64>().unwrap());
        let return_flag: Rc<String> = Rc::new(row[8].to_string());
        let line_status: Rc<String> = Rc::new(row[9].to_string());
        let ship_date: Rc<String> = Rc::new(row[10].to_string());
        let commit_date: Rc<String> = Rc::new(row[11].to_string());
        let receipt_date: Rc<String> = Rc::new(row[12].to_string());
        let shipin_struct: Rc<String> = Rc::new(row[13].to_string());
        let ship_mode: Rc<String> = Rc::new(row[14].to_string());
        let comment: Rc<String> = Rc::new(row[15].to_string());

        let lineitem_rc = LineItemRc::new(order_key, part_key, suppkey, line_number, quantity, extended_price, 
                                     discount, tax, return_flag, line_status, ship_date, commit_date, 
                                     receipt_date, shipin_struct, ship_mode, comment);
        
        lineitems_rc.push(lineitem_rc);
    }
    let elapsed = start.elapsed().as_micros();
    (elapsed, lineitems_rc)
}

impl Serialize for LineItemOwned {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("LineItemOwned", 16)?;
        state.serialize_field("order_key", &self.order_key)?;
        state.serialize_field("part_key", &self.part_key)?;
        state.serialize_field("suppkey", &self.suppkey)?;
        state.serialize_field("line_number", &self.line_number)?;
        state.serialize_field("quantiry", &self.quantity)?;
        state.serialize_field("extended_price", &self.extended_price)?;
        state.serialize_field("discount", &self.discount)?;
        state.serialize_field("tax", &self.tax)?;
        state.serialize_field("return_flag", &self.return_flag)?;
        state.serialize_field("line_status", &self.line_status)?;
        state.serialize_field("ship_date", &self.ship_date)?;
        state.serialize_field("commit_date", &self.commit_date)?;
        state.serialize_field("receipt_date", &self.receipt_date)?;
        state.serialize_field("shipin_struct", &self.shipin_struct)?;
        state.serialize_field("ship_mode", &self.ship_mode)?;
        state.serialize_field("comment", &self.comment)?;
        state.end()
    }
}



impl Serialize for LineItemBorrowed<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("LineItemBorrowed", 16)?;
        state.serialize_field("order_key", &self.order_key)?;
        state.serialize_field("part_key", &self.part_key)?;
        state.serialize_field("suppkey", &self.suppkey)?;
        state.serialize_field("line_number", &self.line_number)?;
        state.serialize_field("quantiry", &self.quantity)?;
        state.serialize_field("extended_price", &self.extended_price)?;
        state.serialize_field("discount", &self.discount)?;
        state.serialize_field("tax", &self.tax)?;
        state.serialize_field("return_flag", &self.return_flag)?;
        state.serialize_field("line_status", &self.line_status)?;
        state.serialize_field("ship_date", &self.ship_date)?;
        state.serialize_field("commit_date", &self.commit_date)?;
        state.serialize_field("receipt_date", &self.receipt_date)?;
        state.serialize_field("shipin_struct", &self.shipin_struct)?;
        state.serialize_field("ship_mode", &self.ship_mode)?;
        state.serialize_field("comment", &self.comment)?;
        state.end()
    }
}


impl Serialize for LineItemRc {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer,
    {
        let mut state = serializer.serialize_struct("LineItemRc", 16)?;
        state.serialize_field("order_key", &self.order_key)?;
        state.serialize_field("part_key", &self.part_key)?;
        state.serialize_field("suppkey", &self.suppkey)?;
        state.serialize_field("line_number", &self.line_number)?;
        state.serialize_field("quantiry", &self.quantity)?;
        state.serialize_field("extended_price", &self.extended_price)?;
        state.serialize_field("discount", &self.discount)?;
        state.serialize_field("tax", &self.tax)?;
        state.serialize_field("return_flag", &self.return_flag)?;
        state.serialize_field("line_status", &self.line_status)?;
        state.serialize_field("ship_date", &self.ship_date)?;
        state.serialize_field("commit_date", &self.commit_date)?;
        state.serialize_field("receipt_date", &self.receipt_date)?;
        state.serialize_field("shipin_struct", &self.shipin_struct)?;
        state.serialize_field("ship_mode", &self.ship_mode)?;
        state.serialize_field("comment", &self.comment)?;
        state.end()
    }
}


// pub fn agg_lineitem_owned_by_order_key(mut linitems: Vec<LineItemOwned>) -> HashMap<i32, Vec<LineItemOwned>> {
//     let size = linitems.len();
//     let mut aggregation: HashMap<i32, Vec<LineItemOwned>> = HashMap::new();
    
//     for _ in 0..size {
//         let lineitem = linitems.pop().unwrap();
//         let key = lineitem.order_key;
//         if !aggregation.contains_key(&key) {
//             aggregation.insert(key, Vec::new());
//         } 
//         aggregation.get_mut(&key).unwrap().push(lineitem);
//     }

//     aggregation
// }

// pub fn agg_lineitem_borrowed_by_order_key(mut linitems: Vec<LineItemBorrowed>) -> HashMap<i32, Vec<LineItemBorrowed>> {
//     let size = linitems.len();
//     let mut aggregation: HashMap<i32, Vec<LineItemBorrowed>> = HashMap::new();
    
//     for _ in 0..size {
//         let lineitem = linitems.pop().unwrap();
//         let key = lineitem.order_key;
//         if !aggregation.contains_key(key) {
//             aggregation.insert(*key, Vec::new());
//         } 
//         aggregation.get_mut(&key).unwrap().push(lineitem);
//     }

//     aggregation
// }

pub fn agg_lineitem_by_order_key<T: LineItem>(mut linitems: Vec<T>) -> HashMap<i32, Vec<T>> {
    let size = linitems.len();
    let mut aggregation: HashMap<i32, Vec<T>> = HashMap::new();
    
    for _ in 0..size {
        let mut lineitem = linitems.pop().unwrap();
        let key = lineitem.get_order_key();
        if !aggregation.contains_key(&key) {
            aggregation.insert(key, Vec::new());
        } 
        aggregation.get_mut(&key).unwrap().push(lineitem);
    }

    aggregation
}