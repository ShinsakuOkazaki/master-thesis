extern crate csv;
extern crate ndarray;
extern crate rand;

use std::error::Error;

use csv::Writer;

use ndarray::prelude::*;
//use ndarray_linalg::Solve;
use rand::{thread_rng, Rng};
use rand::distributions::{Standard};




fn main() -> Result<(), Box<Error>>  {
    let mut wtr = Writer::from_path("random_raw_num.csv")?;
    for _i in 0..1000 {
        let t = generate_tuple();
        wtr.serialize(&t)?;  
    }
    wtr.flush()?;
    Ok(())
}
fn to_tuple(v :&Vec<f32>) -> (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32 ) {
    let t = (v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7], v[8], v[9]);
    return t;
}

fn generate_tuple() -> (f32, f32, f32, f32, f32, f32, f32, f32, f32, f32) {
    let rng = thread_rng();
    let x: Vec<f32> = rng.sample_iter(&Standard).take(10).collect();
    let t = to_tuple(&x);
    return t;
}

