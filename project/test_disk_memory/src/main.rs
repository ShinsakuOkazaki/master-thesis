extern crate csv;
#[macro_use(s)]
extern crate ndarray;
extern crate ndarray_linalg;


//use ndarray::{Array, Axis, ArrayBase, stack};
use ndarray::prelude::*;
use ndarray::stack;
use ndarray_linalg::Solve;
use csv::ReaderBuilder;
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::convert::From;
use csv::Writer;


// run like (time cargo run input1.csv output1.csv) 2>&1 | tee log1.txt
fn main() -> Result<(), Box<dyn Error>>  {
    let args: Vec<String> = env::args().collect();
    let file_path = String::from(&args[1]);
    let ret = get_vector_csv(&file_path).unwrap();
    let (v, d)= ret;
    let matrix = Array::from_shape_vec((d[0], d[1]), v).unwrap();
    let x = matrix.slice(s![.., 0..d[1]-2]);
    let y = matrix.slice(s![.., d[1]-1]);
    let x_train = x.slice(s![0..d[0]/10*9, ..]).to_owned();
    let y_train = y.slice(s![0..d[0]/10*9]).to_owned();
    let _x_test = x.slice(s![d[0]/10*9..d[0], ..]).to_owned();
    let _y_test = y.slice(s![d[0]/10*9..d[0]]).to_owned();
    let mut model = LinearRegression::new();
    model.fit(&x_train, &y_train);
    //let prediction = model.predict(&x_test);
    let w = model.w();
    let mut wtr = Writer::from_path(&args[2])?;
    wtr.serialize(&w.to_vec())?;
    wtr.flush()?;
    println!("{:?}",d);
    println!("{:?}",w);
    Ok(())
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



// structure to get vector from deserialization
#[derive(Debug, Deserialize)]
pub struct Row {
    values: Vec<f32>,
}

// structure for LinearRegression
pub struct LinearRegression {
    w: Array<f32, Ix1>,
}


// implenent functions new(constructor), fit, predict.
impl LinearRegression { 

    // constructor
    pub fn new() -> LinearRegression {
        LinearRegression {
            w: Array::from_vec(vec![0.]),
        }
    }

    // public field of weight
    pub fn w(&self) -> &Array<f32, Ix1> {&self.w}
    

    pub fn fit(&mut self, x: &Array<f32, Ix2>, y: &Array<f32, Ix1>) {

        // get number of rows of x
        let x_len = x.len_of(Axis(0));

        // create 1 dimentional array whose elements are all 1.0.
        // It is used for intercept.
        let ones_arr = Array::from_elem((x_len, 1), 1.);
        // Insert column of all 1 to position of first column
        let xtil = stack(Axis(1), &[ones_arr.view(), x.view()]).unwrap();
        // dot product 
        let a = xtil.t().dot(&xtil);
        let b = xtil.t().dot(y);
        // solve linear equation
        self.w = a.solve_into(b).unwrap();
    }

    
    pub fn predict(&self, x: &Array<f32, Ix2>) -> Array<f32, Ix1> {
        let x_len = x.len_of(Axis(0));
        let ones_arr = Array::from_elem((x_len, 1), 1.);
        let xtil = stack(Axis(1), &[ones_arr.view(), x.view()]).unwrap();
        let prediction = xtil.dot(&self.w);
        prediction
    }
}


