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
use std::collections::{HashMap, BinaryHeap};
use std::cmp::{Ordering, max};
use crossbeam;
use std::fmt::Debug;

const MAX_THREADS: usize = 4;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Started");
    let mut idx = 1;
    let method: i32 = args[idx].parse::<i32>().unwrap();
    println!("Method: {:?}", method);
    idx += 1;
    let k = args[idx].parse::<usize>().unwrap();
    println!("K: {:?}", k);
    idx += 1;
    let n_neighbors = args[idx].parse::<usize>().unwrap();
    println!("N_neighbors: {:?}", n_neighbors);
    idx +=1;
    let n_batch = args[idx].parse::<usize>().unwrap(); 
    println!("N_batch: {:?}", n_batch);
    idx += 1;
    let mut f_trains = Vec::with_capacity(MAX_THREADS);
    for _ in 0..MAX_THREADS {
        let f_train = args[idx].clone();
        f_trains.push(f_train);
        idx +=1;
    }

    let mut f_tests = Vec::with_capacity(MAX_THREADS); 
    for _ in 0..MAX_THREADS {
        let f_test = args[idx].clone();
        f_tests.push(f_test);
        idx += 1;
    }

    let mut n_line_trains = Vec::with_capacity(MAX_THREADS); 
    for _ in 0..MAX_THREADS {
        let n_line_train = args[idx].parse::<usize>().unwrap();
        n_line_trains.push(n_line_train);
        idx += 1;
    }

    let mut n_line_tests = Vec::with_capacity(MAX_THREADS);
    for _ in 0..MAX_THREADS {
        let n_line_test = args[idx].parse::<usize>().unwrap();
        n_line_tests.push(n_line_test);
        idx += 1;
    }

    run(method, k, n_neighbors, &f_trains[..], &f_tests[..], &n_line_trains[..], &n_line_tests[..], n_batch);
}

fn run(method: i32, k: usize, n_neighbors: usize, f_trains: &[String], f_tests: &[String], n_line_trains: &[usize], n_line_tests: &[usize], n_batch: usize) {
    match method {
        1 => run_ex_deepcopy(k, n_neighbors, f_trains, f_tests, n_line_trains, n_line_tests, n_batch),
        _ => println!("Wrong input")
    }
}

fn run_ex_deepcopy(k: usize, n_neighbors: usize, f_trains: &[String], f_tests: &[String], n_line_trains: &[usize], n_line_tests: &[usize], n_batch: usize) {
    let (predictions, preprocess_times, query_times, prediction_times, elapsed_thread) = k_nearest_neighbors_multithread(k, n_neighbors, f_trains, f_tests, n_line_trains, n_line_tests, n_batch);
    println!("{:?}", prediction_times);
    write_to_file("deepcopy", k, n_neighbors, &n_line_trains[..], &n_line_tests[..], n_batch, &preprocess_times[..], &query_times[..], &prediction_times[..], elapsed_thread)
}

fn append_output<T>(output: &mut String, results: &[T])
    where T: Debug
{
    for i in 0..MAX_THREADS {
        let temp = format!("#{:?}", results[i]);
        output.push_str(&temp);
    }
}

fn write_to_file(method: &str, k: usize, n_neighbors:usize, n_line_trains: &[usize], n_line_tests: &[usize], n_batch: usize,
                 preprocess_times: &[u128], query_times: &[u128], prediction_times: &[u128], elapsed_thread: u128) {
    let mut output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}", method, k, n_neighbors, n_batch);
    append_output(&mut output, n_line_trains);
    append_output(&mut output, n_line_trains);
    append_output(&mut output, preprocess_times);
    append_output(&mut output, query_times);
    append_output(&mut output, prediction_times);
    output = format!("{}#{:?}", output, elapsed_thread);
    println!("{}",output);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");
}

fn k_nearest_neighbors_multithread(k: usize, n_neighbors:usize, f_trains: &[String], f_tests: &[String], n_line_trains: &[usize], n_line_tests: &[usize], n_batch: usize) -> (Vec<Vec<String>>, Vec<u128>, Vec<u128>, Vec<u128>, u128){
    let start_thread = Instant::now();
    let (predictions, preprocess_time, query_time, prediction_time) = crossbeam::scope(|scope| {
        let mut handlers = Vec::with_capacity(MAX_THREADS);
        for i in 0..MAX_THREADS {
            let handler = scope.spawn(move |_| {
                k_nearest_neighbors(k, n_neighbors, &f_trains[i], &f_tests[i], n_line_trains[i], n_line_tests[i], n_batch)
            });
            handlers.push(handler);
        }
        let mut local_predictions = Vec::with_capacity(MAX_THREADS);
        let mut batch_preprocess_times = Vec::with_capacity(MAX_THREADS); 
        let mut batch_query_times = Vec::with_capacity(MAX_THREADS);
        let mut select_prediction_times = Vec::with_capacity(MAX_THREADS);
        for handler in handlers {
            let local = handler.join().unwrap();
            let (local_prediction, batch_preprocess_time, batch_query_time, select_prediction_time) = local;
            local_predictions.push(local_prediction);
            batch_preprocess_times.push(batch_preprocess_time);
            batch_query_times.push(batch_query_time);
            select_prediction_times.push(select_prediction_time);
        }
        (local_predictions, batch_preprocess_times, batch_query_times, select_prediction_times)
    }).unwrap();
    let elapsed_thread = start_thread.elapsed().as_millis();
    (predictions, preprocess_time, query_time, prediction_time, elapsed_thread)
}

fn get_batch_size(n_line: usize, n_batch: usize) -> Vec<usize> {
    let mut batch_size_trains = Vec::with_capacity(n_batch);
    let base_size: usize = n_line / n_batch;
    let remainder: usize = n_line % n_batch; 
    for _ in 0..remainder {
        batch_size_trains.push(base_size + 1);
    }
    for _ in remainder..n_batch {
        batch_size_trains.push(base_size);
    }
    batch_size_trains
}

fn k_nearest_neighbors(k: usize, n_neighbors:usize, f_train: &str, f_test: &str, n_line_train: usize, n_line_test: usize, n_batch: usize) -> (Vec<String>, u128, u128 , u128) {

    let path_train = Path::new(&f_train);
    let path_test = Path::new(&f_test);
    let file_train  = File::open(path_train).unwrap();
    let file_test  = File::open(path_test).unwrap();
    let buf_reader_train = BufReader::new(file_train);
    let buf_reader_test = BufReader::new(file_test);
    let mut lines_train = buf_reader_train.lines().peekable();
    let mut lines_test = buf_reader_test.lines().peekable();

    let batch_size_trains = get_batch_size(n_line_train, n_batch);
    let batch_size_tests = get_batch_size(n_line_test, n_batch);
    let mut prediction = Vec::with_capacity(n_batch);

    let mut batch_preprocess_time = 0;
    let mut batch_query_time = 0;
    let mut select_prediction_time = 0;
    for i in 0..n_batch {

        let start_batch_preproccess = Instant::now();
        let batch_train = read_n_lines(&mut lines_train, batch_size_trains[i]).unwrap();
        let batch_test = read_n_lines(&mut lines_test, batch_size_tests[i]).unwrap();
        let train_words = split_documents(&batch_train[..]);
        let test_words = split_documents(&batch_test[..]); 
        let top_k = feature_map(&train_words[..], k);
        let train = create_id_numeric(&train_words[..], &top_k);
        let test = create_id_numeric(&test_words[..], &top_k);
        let (x_source_train, y_source_train) = split_x_y(&train[..]);
        let (x_source_test, _y_source_test) = split_x_y(&test[..]);
        println!("Vectorize started");
        let x_train = vectorize_x(&x_source_train[..]);
        let x_test = vectorize_x(&x_source_test[..]);
        let (y_train, decode_map)= vectorize_y(&y_source_train[..]);
        let elapsed_batch_preprocess = start_batch_preproccess.elapsed().as_millis();
        println!("Vectorize done!");
        let start_batch_query = Instant::now();
        let (_similarities, labels) = knn(&x_train, &y_train, &x_test, n_neighbors);
        let elapsed_batch_query = start_batch_query.elapsed().as_millis();

        let start_select_prediction = Instant::now();
        let label = select_neighbor(&labels);
        let mut id = get_id_from_label(&label, &decode_map);
        let elapsed_select_prediction = start_select_prediction.elapsed().as_millis();
        batch_preprocess_time += elapsed_batch_preprocess;
        batch_query_time += elapsed_batch_query;
        select_prediction_time += elapsed_select_prediction;

        prediction.append(&mut id);
    }
    (prediction, batch_preprocess_time, batch_query_time, select_prediction_time)
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

fn read_n_lines(lines: &mut Peekable<Lines<BufReader<File>>>, n: usize) -> io::Result<Vec<String>> {
    let mut arr = Vec::new();
    let mut count = 0;
    while count < n && lines.peek().is_some() {
        let line = lines.next().unwrap()?;
        arr.push(line);
        count +=1;
    }
    Ok(arr)
}

