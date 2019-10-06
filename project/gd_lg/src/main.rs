#[macro_use(s)]
extern crate ndarray;
extern crate rand;
extern crate ndarray_linalg;

use ndarray::prelude::*;
use ndarray::stack;
use rand::{thread_rng, Rng};
use rand::distributions::{Standard};
use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // let file_path = String::from(&args[1]);
    // let ret = get_vector_csv(&file_path).unwrap();
    let ret = get_vector_csv(&String::from("../data/input3.csv")).unwrap();
    let (v, d)= ret;
    let matrix = Array::from_shape_vec((d[0], d[1]), v).unwrap();
    let x = matrix.slice(s![.., 0..d[1]-2]);
    let y = matrix.slice(s![.., d[1]-1]);
    let x_train = x.slice(s![0..d[0]/10*9, ..]).to_owned();
    let y_train = y.slice(s![0..d[0]/10*9]).to_owned();
    let _x_test = x.slice(s![d[0]/10*9..d[0], ..]).to_owned();
    let _y_test = y.slice(s![d[0]/10*9..d[0]]).to_owned();
    let w = gradiant_descent(&x_train, &y_train, 0.001, 1000, 0.00001);
    //println!{"{:?}", w};
}

fn gradiant_descent(x: &Array<f32, Ix2>, y: &Array<f32, Ix1>, learning_rate: f32, max_iter: i32, eps: f32)
    -> Array<f32, Ix1>  {
    let w_len = x.shape()[1];
    //let w_all = Vec<Array<f32, Ix1>>; // this can be vector of array, because we now number of coefficient
    let mut count_iter = 0;
    let rng = thread_rng();
    let mut w_prev: Array<f32, Ix1> = Array::from_vec(rng.sample_iter(&Standard).take(w_len).collect());
    let mut step_size = 1.0;
    let mut w_cur: Array<f32, Ix1>;
    while count_iter < max_iter && eps < step_size {
        let xtx = x.t().dot(x);
        let wtxtx = w_prev.t().dot(&xtx);
        let ytx = y.t().dot(x);
        let j = wtxtx / ytx;
        w_cur = &w_prev - &j.mapv(|j| j*learning_rate);
        step_size = (&w_prev - &w_cur).sum().abs() / (w_len as f32);
        println!{"Iteration: {}", count_iter}
        println!("{:?}", w_cur);
        count_iter += 1;
        w_prev = w_cur;
    }
    return w_prev;
}

// structure to get vector from deserialization
#[derive(Debug, Deserialize)]
pub struct Row {
    values: Vec<f32>,
}

// function to get a vector from csv file. Every row is combined into one long vector
// and later it will be reshaped.
fn get_vector_csv(file :&String) -> Result<(Vec<f32>, [usize;2]), Box<dyn Error>> {
    
    // Configulate file reader and load from csv file in binary
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(file)?;    
    
    // deserialize and make iterator whose element is each row
    let mut iter = rdr.deserialize();

    // append rows to x until iterator returns None
    let mut x = Vec::new();
    let mut dimention: [usize; 2] = [0, 0];
    loop {
        if let Some(result) = iter.next() {
            dimention[0] += 1;
            let row: Row = result?;
            let mut record = row.values;
            if dimention[0] == 1 {
                dimention[1] = record.len();
            }
            x.append(&mut record);
        } else {
            break;
        }
    }  
    let ret_val = (x, dimention);
    return Ok(ret_val);
}




// // structure for LinearRegression
// pub struct LinearRegression {
//     w: Array<f32, Ix1>,
// }

// impl LinearRegression {
//     pub fn new() -> LinearRegression {
//         LinearRegression {
//             w: Array::from_vec(vec![0.]),
//         }
//     }

//     // public field of weight
//     pub fn w(&self) -> &Array<f32, Ix1> {&self.w} 

//     pub fn fit(&mut self, x: &Array<f32, Ix2>, y: &Array<f32, Ix1>) {
//         // get number of rows of x
//         let x_len = x.len_of(Axis(0));
//         // create 1 dimentional array whose elements are all 1.0.
//         // It is used for intercept.
//         let ones_arr = Array::from_elem((x_len, 1), 1.);
//         // Insert column of all 1 to position of first column
//         let xtil = stack(Axis(1), &[ones_arr.view(), x.view()]).unwrap();
//     }
// }