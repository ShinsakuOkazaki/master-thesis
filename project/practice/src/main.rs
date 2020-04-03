use std::path::Path;
use std::env;
mod module;
use module::deeptf::*;
use module::rctf::*;
use ndarray::{Array, ArrayView, ArrayViewMut, s, Ix1, Ix2};
use std::collections::{HashMap, BinaryHeap};
use std::cmp::{Ordering, max};
use std::rc::Rc;
const MAX_THREADS: usize = 4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let method: i32 = args[1].parse::<i32>().unwrap();
    let train_file = Path::new(&args[2]);
    let test_file = Path::new(&args[3]);
    let k: usize = args[4].parse::<usize>().unwrap();
    let n_neighbors: usize = args[5].parse::<usize>().unwrap();
    run(method, k, n_neighbors, &train_file, &test_file);
}

fn run(method: i32, k: usize, n_neighbors: usize, train_file: &Path, test_file: &Path) {
    match method {
        1 => run_ex_deepcopy(k, n_neighbors, train_file, test_file), 
        2 => run_ex_rc(k, n_neighbors, train_file, test_file),
        _ => println!("Wrong input")
    }
}

fn run_ex_deepcopy(k: usize, n_neighbors:usize, train_file: &Path, test_file: &Path) {
    let prediction = k_nearest_neighbors(k, n_neighbors, train_file, test_file);
    println!("{:?}", prediction);
}

fn run_ex_rc(k: usize, n_neighbors:usize, train_file: &Path, test_file: &Path) {
    let prediction = k_nearest_neighbors_with_rc(k, n_neighbors, train_file, test_file);
    println!("{:?}", prediction);
}

//fn knearest_neighbors_multithread(k: usize, n_neighbors:usize, train_files: &[Path], test_file: &[Path]) {
//
//}

// fn combine_neighbors(similarities: &[Array<f64, Ix2>], ids: &[Vec<String>]) {
//     let (n, m) = similarities[0].dim();
//     let mut res_similarities = Array::zeros((n, m));
// }

fn k_nearest_neighbors(k: usize, n_neighbors:usize, train_file: &Path, test_file: &Path) -> (Array<f64, Ix2>, Vec<String>) {
    let train_words = split_documents(train_file);
    let test_words = split_documents(test_file);
    let top_k = feature_map(&train_words[..], k);
    let train =  create_id_numeric(&train_words[..], &top_k);
    let test = create_id_numeric(&test_words[..], &top_k);
    let x_train = vectorize_x(&train[..]);
    let x_test = vectorize_x(&test[..]);
    let (y_train, decode_map)= vectorize_y(&train[..]);
    let (similarities, labels) = knn(&x_train, &y_train, &x_test, n_neighbors);
    let label = select_neighbor(&labels);
    let id = get_id_from_label(&label, &decode_map);
    (similarities, id)
}



fn k_nearest_neighbors_with_rc(k: usize, n_neighbors: usize, train_file: &Path, test_file: &Path) -> Vec<Rc<String>> {
    let train_words = split_documents_with_rc(train_file);
    let test_words = split_documents_with_rc(test_file);
    let top_k = feature_map_with_rc(&train_words[..], k);
    let train =  create_id_numeric_with_rc(&train_words[..], &top_k);
    let test = create_id_numeric_with_rc(&test_words[..], &top_k);
    let x_train = vectorize_x_with_rc(&train[..]);
    let x_test = vectorize_x_with_rc(&test[..]);
    let (y_train, decode_map)= vectorize_y_with_rc(&train[..]); 
    let (_similarities, labels) = knn(&x_train, &y_train, &x_test, n_neighbors);
    let label = select_neighbor(&labels);
    let id = get_id_from_label_with_rc(&label, &decode_map);
    (similarities, id)
}

fn select_neighbor(labels: &Array<i32, Ix2>) -> Array<i32, Ix1> {
    let (n, m) = labels.dim();
    let mut res = Array::zeros(n);
    for i in 0..n {
        let mut map_count = HashMap::new();
        let raw = labels.slice(s![i, ..]);
        for j in 0..m {
            let count = map_count.entry(raw[j]).or_insert(0);
            *count += 1;
        }
        let mut iter = map_count.iter(); 
        let (k, v) = iter.next().unwrap();
        let mut nearest_neighbor = *k;
        let mut maximum = *v;
        while let Some((k, v)) = iter.next()  {
            if maximum < *v {
                maximum = max(maximum, *v);
                nearest_neighbor = *k;
            }
        }
        res[i] = nearest_neighbor;
    }
    res
}


fn knn(x_train: &Array<f64, Ix2>, y_train: &Array<i32, Ix1>, x_test: &Array<f64, Ix2>, n_neighbors: usize) -> (Array<f64, Ix2>, Array<i32, Ix2>){
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
        pq.push(MinNonNan(i, all_similarity[i]));
        if pq.len() > n_neighbors {
            pq.pop();
        }
    }
    let mut pair;
    let mut indices = Array::zeros(n_neighbors);
    for i in 0..n_neighbors {
        pair = pq.pop().unwrap();
        similarity[i] = pair.1;
        indices[i] = pair.0;
    }
    indices
}


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