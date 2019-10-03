extern crate csv;
#[macro_use]
 extern crate serde_derive;

extern crate ndarray;
extern crate rand;


use std::error::Error;

use csv::Writer;

use ndarray::prelude::*;
//use ndarray_linalg::Solve;
use rand::{thread_rng, Rng};
use rand::distributions::{Standard};
use std::env;


#[derive(Serialize)]
pub struct Row {
    values: Vec<f32>,
}

// path row number and column number on command line arguments ex: cargo run filename 1000 10
fn main() -> Result<(), Box<Error>>  {
    let args: Vec<String> = env::args().collect();
    let row_num = args[2].parse::<i32>()?;
    let col_num = args[3].parse::<usize>()?;
    let rng = thread_rng();
    let mut wtr = Writer::from_path(&args[1])?;
    for _i in 0..row_num {
        let x: Vec<f32> = rng.sample_iter(&Standard).take(col_num).collect();
        wtr.serialize(&x)?;
    }
    wtr.flush()?;
    Ok(())
}
// fn to_array(v :&Vec<f32>) {
//     const v_len: f32 = v.len();
//     let mut a: [f32; v_len];
//     for i in 0..v_len {
//         a[i] = v[i];
//     }
//     a
// }

// fn generate_array() -> [f32] {
    
    
//     let a = to_array(&x);
//     a
//}



// fn main() -> Result<(), Box<Error>>  {
//     let mut wtr = Writer::from_path("random_raw_num.csv")?;
//     for _i in 0..1000 {
//         let t = generate_tuple();
//         wtr.serialize(&t)?;  
//     }
//     wtr.flush()?;
//     Ok(())
// }
// fn to_tuple(v :&Vec<f32>) -> (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32 ) {
//     let t = (v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9]);
//     return t;
// }

// fn generate_tuple() -> (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) {
//     let rng = thread_rng();
//     let x: Vec<f32> = rng.sample_iter(&Standard).take(10).collect();
//     let t = to_tuple(&x);
//     return t;
// }

