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
        //println!("{:#?}", &t)
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


// let rng = thread_rng();
    // let mut x0: Vec<f32> = rng.sample_iter(&Standard).take(1000).collect();
    // let x1: Vec<f32> = rng.sample_iter(&Standard).take(1000).collect();
    // let x2: Vec<f32> = rng.sample_iter(&Standard).take(1000).collect();
    // let x3: Vec<f32> = rng.sample_iter(&Standard).take(1000).collect();
    // let x4: Vec<f32> = rng.sample_iter(&Standard).take(1000).collect();
    // let x5: Vec<f32> = rng.sample_iter(&Standard).take(1000).collect();
    // let x6: Vec<f32> = rng.sample_iter(&Standard).take(1000).collect();
    // let x7: Vec<f32> = rng.sample_iter(&Standard).take(1000).collect();
    // let x8: Vec<f32> = rng.sample_iter(&Standard).take(1000).collect();
    // let x9: Vec<f32> = rng.sample_iter(&Standard).take(1000).collect();

    // let mut columns = [x1, x2, x3, x4, x5, x6, x7, x8, x9];

    // for c in &mut columns {
    //     x0.append(c);
    // }

    // let x = Array::from_shape_vec((1000, 10), x0).unwrap();

    //  println!("{:#?}", &x[0]);

    // let mut wtr = Writer::from_path("random_raw_num.csv")?;
    // for r in &mut x {
    //     let t = to_tuple(r);
    //     wtr.serialize(&t)?;   
    // }
    // wtr.flush()?;
    // Ok(())

    // let rows = Array::from_shape_vec((1000, 10), x0).unwrap();
    // println!("{:#?}", &rows);

    // let tuples: Vec<(f32, f32, f32, f32, f32, f32, f32, f32, f32, f32 )>;
    // for row in &rows {
    //     let t = to_tuple(row);
    //     tuples.append(&t);
    // }
}


// fn csv_writer(columns: &mut [Vec<f32>;10]) -> Result<(), Box<Error>> {
//     let mut wtr = Writer::from_path("random_raw_num.csv")?;
//     for x in columns {
//             wtr.write_record(x);
//     }
//     wtr.flush()?;
//     Ok(())
// }
