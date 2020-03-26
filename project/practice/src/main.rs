extern crate ndarray;
use std::path::Path;
use std::env;
mod module;
use module::deeptf::*;
use ndarray::prelude::*;
use std::collections::HashMap;
fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let k: usize = args[2].parse::<usize>().unwrap();
    let id_words = split_documents(path);
    let top_k = feature_map(&id_words[..], k);
    let id_numeric =  create_id_numeric(&id_words[..], &top_k);
    let x = vectorize_x(&id_numeric[..]);
    let y = vectorize_y(&id_numeric[..]);
    println!("{:?}", x);
    println!("{:?}", y);
}

fn vectorize_x(source: &[(String, Vec<f64>)]) -> Array<f64, Ix2>{
    let n = source.len();
    let m = source[0].1.len();
    let mut vector = Vec::with_capacity(n * m);
    for i in 0..n {
        vector.extend_from_slice(&source[i].1[..]);
    }
    
    let x = Array::from_shape_vec((n, m), vector).unwrap();
    x
}

fn vectorize_y(source: &[(String, Vec<f64>)]) -> Array<f64, Ix1> {
    let n = source.len();
    let mut map = HashMap::new();
    let mut id;
    let mut encode = 0.0;
    for i in 0..n {
        id = source[i].0.clone();
        if !map.contains_key(&id) {
            map.insert(id, encode);
            encode += 1.0;
        }
    }

    let mut y = Array::zeros(n);
    for i in 0..n {
        encode = *map.get(&source[i].0).unwrap();
        y[i] = encode;
    }
    y
}







