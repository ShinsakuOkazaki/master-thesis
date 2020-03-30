use std::path::Path;
use std::env;
mod module;
use module::deeptf::*;
//use module::rctf::*;
use ndarray::{Array, ArrayView, ArrayViewMut, s, Ix1, Ix2};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::{Reverse, Ordering};


fn main() {
    let args: Vec<String> = env::args().collect();
    let train_file = Path::new(&args[1]);
    let test_file = Path::new(&args[2]);
    let k: usize = args[3].parse::<usize>().unwrap();
    let n_neighbors: usize = args[4].parse::<usize>().unwrap();
    let train_words = split_documents(train_file);
    let test_words = split_documents(test_file);
    let top_k = feature_map(&train_words[..], k);
    let train =  create_id_numeric(&train_words[..], &top_k);
    let test = create_id_numeric(&test_words[..], &top_k);
    let x_train = vectorize_x(&train[..]);
    let x_test = vectorize_x(&test[..]);
    let y_train = vectorize_y(&train[..]);
    let (similarities, labels) = get_neighbors(&x_train, &y_train, &x_test, n_neighbors);
    println!("{:?}", similarities);
    println!("{:?}", labels);
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

fn vectorize_y(source: &[(String, Vec<f64>)]) -> Array<i32, Ix1> {
    let n = source.len();
    let mut encode_map = HashMap::new();
    //let mut decode_map = Vec::new();
    let mut id;
    let mut encode = 0;
    for i in 0..n {
        id = source[i].0.clone();
        if !encode_map.contains_key(&id) {
            encode_map.insert(id, encode);
            //decode_map[encode] = id;
            encode += 1;
        }
    }

    let mut y = Array::zeros(n);
    for i in 0..n {
        encode = *encode_map.get(&source[i].0).unwrap();
        y[i] = encode;
    }
    y
}


fn get_neighbors(x_train: &Array<f64, Ix2>, y_train: &Array<i32, Ix1>, x_test: &Array<f64, Ix2>, n_neighbors: usize) -> (Array<f64, Ix2>, Array<i32, Ix2>){
    let m = x_test.shape()[0];
    let mut labels = Array::zeros((m, n_neighbors));
    let mut similarities = Array::zeros((m, n_neighbors));
    for i in 0..m {
        get_neighbors_of_raw(x_train, y_train, x_test.slice(s![i, ..]), similarities.slice_mut(s![i, ..]), labels.slice_mut(s![i, ..]), n_neighbors);
    }
    (similarities, labels)
}

fn get_neighbors_of_raw(x_train: &Array<f64, Ix2>, y_train: &Array<i32, Ix1>, x_test_raw: ArrayView<f64, Ix1>, similarity: ArrayViewMut<f64, Ix1>, mut label: ArrayViewMut<i32, Ix1>, n_neighbors: usize) {
    let all_similarity = x_train.dot(&x_test_raw);
    let indices = get_n_similarity_and_index(&all_similarity, similarity, n_neighbors);
    for i in 0..n_neighbors {
        label[i] = y_train[indices[i]];
    }
}

fn get_n_similarity_and_index(all_similarity: &Array<f64, Ix1>, mut similarity: ArrayViewMut<f64, Ix1>, n_neighbors: usize) -> Array<usize, Ix1> {
    let mut pq = BinaryHeap::with_capacity(n_neighbors);
    let n = all_similarity.shape()[0];
    for i in 0..n {
        pq.push(Reverse(MinNonNan(i, all_similarity[i])));
        if pq.len() > n_neighbors {
            pq.pop();
        }
    }
    let mut pair;
    let mut indices = Array::zeros(n_neighbors);
    for i in 0..n_neighbors {
        pair = pq.pop().unwrap().0;
        similarity[i] = pair.1;
        indices[i] = pair.0;
    }
    indices
}

//type MinNonNan = Reverse<NotNan<f64>>;

#[derive(PartialEq)]
struct MinNonNan(usize, f64);

impl Eq for MinNonNan {}

impl PartialOrd for MinNonNan {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl Ord for MinNonNan {
    fn cmp(&self, other: &MinNonNan) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}