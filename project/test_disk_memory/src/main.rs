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

use std::error::Error;

fn main()  {
    let file_path = String::from("random_raw_num.csv");
    let v = get_vector_csv(&file_path).unwrap();
    let matrix = Array::from_shape_vec((1000, 10), v).unwrap();;

    let x = matrix.slice(s![.., 0..8]);
    let y = matrix.slice(s![.., 9]);

    let x_train = x.slice(s![0..900, ..]).to_owned();
    let y_train = y.slice(s![0..900]).to_owned();
    let x_test = x.slice(s![900..1000, ..]).to_owned();
    let y_test = y.slice(s![900..1000]).to_owned();

    let mut model = LinearRegression::new();
    model.fit(&x_train, &y_train);
    let prediction = model.predict(&x_test);
    let w = model.w();
    println!("{:?}", prediction);

}

// function to get a vector from csv file. Every row is combined into one long vector
// and later it will be reshaped.
fn get_vector_csv(file :&String) -> Result<Vec<f32>, Box<dyn Error>> {
    
    // Configulate file reader and load from csv file in binary
    let mut rdr = ReaderBuilder::new()
        .has_headers(false)
        .from_path(file)?;    
    
    // deserialize and make iterator whose element is each row
    let mut iter = rdr.deserialize();

    // append rows to x until iterator returns None
    let mut x = Vec::new();
    loop {
        if let Some(result) = iter.next() {
            let row: Row = result?;
            let mut record = row.values;
            x.append(&mut record);
        } else {
            break;
        }
    }  
    return Ok(x);
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
        let w = a.solve_into(b).unwrap();
        self.w =  w.clone();
    }

    
    pub fn predict(&self, x: &Array<f32, Ix2>) -> Array<f32, Ix1> {
        let x_len = x.len_of(Axis(0));
        let ones_arr = Array::from_elem((x_len, 1), 1.);
        let xtil = stack(Axis(1), &[ones_arr.view(), x.view()]).unwrap();
        let prediction = xtil.dot(&self.w);
        prediction
    }
}
