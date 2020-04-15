use std::path::Path;
use std::env;
mod module;
use module::deeptf::*;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Read};
use std::iter::{Peekable, Iterator};
use ndarray::prelude::*;
use ndarray::{
    Data,
    RemoveAxis,
    Zip,
};
use std::cmp::Ordering;
use std::ptr::copy_nonoverlapping;

const MAX_THREADS: usize = 4;

fn main() {
    let a = Array::linspace(0., 63., 64).into_shape((8, 8)).unwrap();
    let strings = a.map(|x| x.to_string());

    let perm = a.sort_axis_by(Axis(1), |i, j| {
        a[[i, 0]] > a[[j, 0]]
    });
    println!("{:?}", perm);
    let b = a.permute_axis(Axis(0), &perm);
    println!("{:?}", b);

    println!("{:?}", strings);
    let c = strings.permute_axis(Axis(1), &perm);
    println!("{:?}", c);
    // let args: Vec<String> = env::args().collect();
    // let method: i32 = args[1].parse::<i32>().unwrap();
    // let k: usize = args[2].parse::<usize>().unwrap();
    // let n_neighbors: usize = args[3].parse::<usize>().unwrap();
    // let mut train_files = Vec::with_capacity(MAX_THREADS);
    // let mut test_file = args[4].clone();
    // for i in 5..5 + MAX_THREADS {
    //     train_files.push(args[i].clone());
    // }
    // run(method, k, n_neighbors, &train_files, &test_file);
}

#[derive(Clone, Debug)]
pub struct Permutation {
    indices: Vec<usize>,
}

impl Permutation {
    /// Checks if the permutation is correct
    pub fn from_indices(v: Vec<usize>) -> Result<Self, ()> {
        let perm = Permutation { indices: v };
        if perm.correct() {
            Ok(perm)
        } else {
            Err(())
        }
    }

    fn correct(&self) -> bool {
        let axis_len = self.indices.len();
        let mut seen = vec![false; axis_len];
        for &i in &self.indices {
            if seen[i] {
                return false;
            }
            seen[i] = true;
        }
        true
    }
}

pub trait SortArray {
    /// ***Panics*** if `axis` is out of bounds.
    fn identity(&self, axis: Axis) -> Permutation;
    fn sort_axis_by<F>(&self, axis: Axis, less_than: F) -> Permutation
        where F: FnMut(usize, usize) -> bool;
}

pub trait PermuteArray {
    type Elem;
    type Dim;
    fn permute_axis(self, axis: Axis, perm: &Permutation)
        -> Array<Self::Elem, Self::Dim>
        where Self::Elem: Clone, Self::Dim: RemoveAxis;
}

impl<A, S, D> SortArray for ArrayBase<S, D>
    where S: Data<Elem=A>,
          D: Dimension,
{
    fn identity(&self, axis: Axis) -> Permutation {
        Permutation {
            indices: (0..self.len_of(axis)).collect(),
        }
    }

    fn sort_axis_by<F>(&self, axis: Axis, mut less_than: F) -> Permutation
        where F: FnMut(usize, usize) -> bool
    {
        let mut perm = self.identity(axis);
        perm.indices.sort_by(move |&a, &b|
            if less_than(a, b) {
                Ordering::Less
            } else if less_than(b, a) {
                Ordering::Greater
            } else {
                Ordering::Equal
            });
        perm
    }
}

impl<A, D> PermuteArray for Array<A, D>
    where D: Dimension,
{
    type Elem = A;
    type Dim = D;

    fn permute_axis(self, axis: Axis, perm: &Permutation) -> Array<A, D>
        where D: RemoveAxis,
    {
        let axis = axis;
        let axis_len = self.len_of(axis);
        assert_eq!(axis_len, perm.indices.len());
        debug_assert!(perm.correct());

        let mut v = Vec::with_capacity(self.len());
        let mut result;

        // panic-critical begin: we must not panic
        unsafe {
            v.set_len(self.len());
            result = Array::from_shape_vec_unchecked(self.dim(), v);
            for i in 0..axis_len {
                let perm_i = perm.indices[i];
                Zip::from(result.subview_mut(axis, perm_i))
                    .and(self.subview(axis, i))
                    .apply(|to, from| {
                        copy_nonoverlapping(from, to, 1)
                    });
            }
            // forget moved array elements but not its vec
            let mut old_storage = self.into_raw_vec();
            old_storage.set_len(0);
            // old_storage drops empty
        }
        // panic-critical end
        result
    }
}

fn write_to_file(method: &str, k: usize, n_neighbors:usize, elapsed_threads: u128, elapsed_global: u128) {
    let output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}#{:?}\n", 
                         method, k, n_neighbors, elapsed_threads, elapsed_global);
    println!("{}",output);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");
}

fn run(method: i32, k: usize, n_neighbors: usize, train_files: &[String], test_file: &String) {
    match method {
        1 => run_ex_deepcopy(k, n_neighbors, train_files, test_file), 
        2 => run_ex_arc(k, n_neighbors, train_files, test_file),
        _ => println!("Wrong input")
    }
}

fn run_ex_deepcopy( k: usize, n_neighbors:usize, train_files: &[String], test_file: &String) {
    let (prediction, elapsed_threads, elapsed_global)= k_nearest_neighbors_multithread(k, n_neighbors, train_files, test_file);
    write_to_file("deepcopy", k, n_neighbors, elapsed_threads, elapsed_global);
    println!("{:?}", prediction);
}

fn run_ex_arc(k: usize, n_neighbors:usize, train_files: &[String], test_file: &String) {
     let (prediction, elapsed_threads, elapsed_global) = k_nearest_neighbors_multithread_with_arc(k, n_neighbors, train_files, test_file);
     write_to_file("arc", k, n_neighbors, elapsed_threads, elapsed_global);
     println!("{:?}", prediction);
}

fn k_nearest_neighbors_multithread(k: usize, n_neighbors:usize, train_files: &[String], test_file: &String) -> (Vec<String>, u128, u128){
    let start_threads = Instant::now();
    let (mut similarities, mut ids)= crossbeam::scope(|scope| {
        let mut handlers = Vec::with_capacity(MAX_THREADS);
        for i in 0..MAX_THREADS {
            let handler = scope.spawn(move |_| {
                k_nearest_neighbors(k, n_neighbors, &train_files[i], &test_file)
            });
            handlers.push(handler);
        }
        let mut local_similarities = Vec::with_capacity(MAX_THREADS);
        let mut local_ids = Vec::with_capacity(MAX_THREADS); 
        for handler in handlers {
            let local: (Array<f64, Ix2>, Vec<Vec<String>>) = handler.join().unwrap();
            let (local_similarity, local_id)= local;
            local_similarities.push(local_similarity);
            local_ids.push(local_id);
        }
        (local_similarities, local_ids)
    }).unwrap();
    let elapsed_threads = start_threads.elapsed().as_millis();
    let start_combine = Instant::now();
    let (combined_similarities, combined_ids) = combine_neighbors(similarities, ids);
    let (combined_labels, decode_map) = vectorize_ids(&combined_ids[..]);
    let (n_similarities, n_labels) = select_neighbors_combine(&combined_similarities, &combined_labels, n_neighbors);
    let label = select_neighbor(&n_labels);
    let id = get_id_from_label(&label, &decode_map);
    let elapsed_global = start_combine.elapsed().as_millis();
    (id, elapsed_threads, elapsed_global)
}


fn k_nearest_neighbors_multithread_with_arc(k: usize, n_neighbors:usize, train_files: &[String], test_file: &String) ->(Vec<Arc<String>>, u128, u128) {
    let start_threads = Instant::now();
    let (mut similarities, mut ids)= crossbeam::scope(|scope| {
        let mut handlers = Vec::with_capacity(MAX_THREADS);
        for i in 0..MAX_THREADS {
            let handler = scope.spawn(move |_| {
                k_nearest_neighbors_with_arc(k, n_neighbors, &train_files[i], &test_file)
            });
            handlers.push(handler);
        }
        let mut local_similarities = Vec::with_capacity(MAX_THREADS);
        let mut local_ids = Vec::with_capacity(MAX_THREADS); 
        for handler in handlers {
            let local = handler.join().unwrap();
            let (local_similarity, local_id)= local;
            local_similarities.push(local_similarity);
            local_ids.push(local_id);
        }
        (local_similarities, local_ids)
    }).unwrap();
    let elapsed_threads = start_threads.elapsed().as_millis();
    let start_combine = Instant::now();
    let (combined_similarities, combined_ids) = combine_neighbors_with_arc(similarities, ids);
    let (combined_labels, decode_map) = vectorize_ids_with_arc(&combined_ids[..]);
    let (n_similarities, n_labels) = select_neighbors_combine(&combined_similarities, &combined_labels, n_neighbors);
    let label = select_neighbor(&n_labels);
    let id = get_id_from_label_with_arc(&label, &decode_map);
    let elapsed_global = start_combine.elapsed().as_millis();
    (id, elapsed_threads, elapsed_global) 
}

fn select_neighbors_combine(combined_similarities: &Array<f64, Ix2>, combined_labels: &Array<i32, Ix2>, n_neighbors: usize) -> (Array<f64, Ix2>, Array<i32, Ix2>){
    let (n, _m) = combined_similarities.dim();
    let mut n_similarities = Array::zeros((n, n_neighbors));
    let mut n_labels = Array::zeros((n, n_neighbors));
    for i in 0..n {
        let indices = get_n_similarity_and_index(&combined_similarities.slice(s![i, ..]), n_similarities.slice_mut(s![i, ..]), n_neighbors);
        for j in 0..indices.len() {
            n_labels[[i, j]] = combined_labels.slice(s![i,..])[indices[j]];
        }
    }
    (n_similarities, n_labels)
}



fn k_nearest_neighbors(k: usize, n_neighbors:usize, train_file: &str, test_file: &str) -> (Array<f64, Ix2>, Vec<Vec<String>>) {
    let train_path = Path::new(train_file);
    let test_path = Path::new(test_file); 
    let train_words = split_documents(&train_path);
    let test_words = split_documents(&test_path);
    let top_k = feature_map(&train_words[..], k);
    let train =  create_id_numeric(&train_words[..], &top_k);
    let test = create_id_numeric(&test_words[..], &top_k);
    let (x_source_train, y_source_train) = split_x_y(&train[..]);
    let (x_source_test, _y_source_test) = split_x_y(&test[..]);
    let x_train = vectorize_x(&x_source_train[..]);
    let x_test = vectorize_x(&x_source_test[..]);
    let (y_train, decode_map)= vectorize_y(&y_source_train[..]);
    let (similarities, labels) = knn(&x_train, &y_train, &x_test, n_neighbors);
    let ids = get_ids_from_labels(&labels, &decode_map);
    (similarities, ids)
}



fn k_nearest_neighbors_with_arc(k: usize, n_neighbors: usize, train_file: &str, test_file: &str) -> (Array<f64, Ix2>, Vec<Vec<Arc<String>>>) {
    let train_path = Path::new(train_file);
    let test_path = Path::new(test_file);
    let train_words = split_documents_with_arc(train_path);
    let test_words = split_documents_with_arc(test_path);
    let top_k = feature_map_with_arc(&train_words[..], k);
    let train =  create_id_numeric_with_arc(&train_words[..], &top_k);
    let test = create_id_numeric_with_arc(&test_words[..], &top_k);
    let (x_source_train, y_source_train) = split_x_y_with_arc(&train[..]);
    let (x_source_test, _y_source_test) = split_x_y_with_arc(&test[..]);
    let x_train = vectorize_x_with_arc(&x_source_train[..]);
    let x_test = vectorize_x_with_arc(&x_source_test[..]);
    let (y_train, decode_map)= vectorize_y_with_arc(&y_source_train[..]); 
    let (similarities, labels) = knn(&x_train, &y_train, &x_test, n_neighbors);
    let ids = get_ids_from_labels_with_arc(&labels, &decode_map);
    (similarities, ids)
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
    let indices = get_n_similarity_and_index(&all_similarity.view(), similarity, n_neighbors);
    for i in 0..n_neighbors {
        label[i] = y_train[indices[i]];
    }
}

fn get_n_similarity_and_index(all_similarity: &ArrayView<f64, Ix1>, mut similarity: ArrayViewMut<f64, Ix1>, n_neighbors: usize) -> Array<usize, Ix1> {
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