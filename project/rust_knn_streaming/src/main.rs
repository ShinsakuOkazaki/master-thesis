use std::path::Path;
use std::env;
mod module;
use module::deeptf::*;
use module::arctf::*;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::iter::{Peekable, Iterator};
use ndarray::prelude::*;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::{Ordering, max};
use crossbeam;
use std::fmt::Debug;
use std::sync::Arc;

// const MAX_THREADS: usize = 10;
const MAX_THREADS: usize = 8;
// const MAX_THREADS: usize = 4;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut idx = 1;
    let method: i32 = args[idx].parse::<i32>().unwrap();
    println!("Method: {:?}", method);
    idx += 1;
    let strategy: usize = args[idx].parse::<usize>().unwrap();
    println!("Strategy: {:?}", strategy);
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

    run(method, strategy, k, n_neighbors, &f_trains[..], &f_tests[..], &n_line_trains[..], &n_line_tests[..], n_batch);
}

fn run(method: i32, strategy: usize, k: usize, n_neighbors: usize, f_trains: &[String], f_tests: &[String], n_line_trains: &[usize], n_line_tests: &[usize], n_batch: usize) {
    match method {
        1 => run_ex_deepcopy(strategy, k, n_neighbors, f_trains, f_tests, n_line_trains, n_line_tests, n_batch),
        2 => run_ex_arc(strategy, k, n_neighbors, f_trains, f_tests, n_line_trains, n_line_tests, n_batch),
        _ => println!("Wrong input")
    }
}

fn run_ex_deepcopy(strategy: usize, k: usize, n_neighbors: usize, f_trains: &[String], f_tests: &[String], n_line_trains: &[usize], n_line_tests: &[usize], n_batch: usize) {
    let (predictions, load_times, preprocess_times, query_times, prediction_times, elapsed_thread) = k_nearest_neighbors_multithread(strategy, k, n_neighbors, f_trains, f_tests, n_line_trains, n_line_tests, n_batch);
    println!("Prediction: {:?}", predictions);
    write_to_file("deepcopy", strategy, k, n_neighbors, &n_line_trains[..], &n_line_tests[..], n_batch, &load_times[..], &preprocess_times[..], &query_times[..], &prediction_times[..], elapsed_thread);
}

fn run_ex_arc(strategy: usize, k: usize, n_neighbors: usize, f_trains: &[String], f_tests: &[String], n_line_trains: &[usize], n_line_tests: &[usize], n_batch: usize) {
    let (predictions, load_times, preprocess_times, query_times, prediction_times, elapsed_thread) = k_nearest_neighbors_multithread_with_arc(strategy, k, n_neighbors, f_trains, f_tests, n_line_trains, n_line_tests, n_batch);
    println!("Prediction: {:?}", predictions);
    write_to_file("arc", strategy, k, n_neighbors, &n_line_trains[..], &n_line_tests[..], n_batch, &load_times[..], &preprocess_times[..], &query_times[..], &prediction_times[..], elapsed_thread);
}

// fn dump_result(predictions: &[Vec<String>], method: &str, k: usize, n_neighbors: usize, n_batch: usize) {
//     let serialized = serde_json::to_string(&predictions).unwrap();
//     let mut file = OpenOptions::new()
//             .write(true)
//             .create(true)
//             .open(format!("prediction/method{}_k{}_nneighbors{}_n_batch{}.txt", method, k, n_neighbors, n_batch))
//             .unwrap();
//     file.write_all(serialized.as_bytes()).expect("Fail to write file.");
// }

// fn dump_result_with_arc(predictions: &[Vec<Arc<String>>], method: &str, k: usize, n_neighbors: usize, n_batch: usize) {
//     let serialized = serde_json::to_string(&predictions).unwrap();
//     let mut file = OpenOptions::new()
//             .write(true)
//             .create(true)
//             .open(format!("prediction/method{}_k{}_nneighbors{}_n_batch{}.txt", method, k, n_neighbors, n_batch))
//             .unwrap();
//     file.write_all(serialized.as_bytes()).expect("Fail to write file.");
// }

fn append_output<T>(output: &mut String, results: &[T])
    where T: Debug
{
    for i in 0..MAX_THREADS {
        let temp = format!("#{:?}", results[i]);
        output.push_str(&temp);
    }
}

fn write_to_file(method: &str, strategy: usize, k: usize, n_neighbors:usize, n_line_trains: &[usize], n_line_tests: &[usize], n_batch: usize,
                 load_times: &[u128], preprocess_times: &[u128], query_times: &[u128], prediction_times: &[u128], elapsed_thread: u128) {
    let mut output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}#{:?}", method, strategy, k, n_neighbors, n_batch);
    append_output(&mut output, n_line_trains);
    append_output(&mut output, n_line_tests);
    append_output(&mut output, load_times);
    append_output(&mut output, preprocess_times);
    append_output(&mut output, query_times);
    append_output(&mut output, prediction_times);
    output = format!("{}#{:?}\n", output, elapsed_thread);
    println!("{}",output);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");
}

fn k_nearest_neighbors_multithread(strategy: usize, k: usize, n_neighbors:usize, f_trains: &[String], f_tests: &[String], n_line_trains: &[usize], n_line_tests: &[usize], n_batch: usize) -> (Vec<Vec<String>>, Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>, u128){
    let start_thread = Instant::now();
    let (predictions, load_time, preprocess_time, query_time, prediction_time) = crossbeam::scope(|scope| {
        let mut handlers = Vec::with_capacity(MAX_THREADS);
        for i in 0..MAX_THREADS {
            let handler = scope.spawn(move |_| {
                k_nearest_neighbors(strategy, k, n_neighbors, &f_trains[i], &f_tests[i], n_line_trains[i], n_line_tests[i], n_batch).unwrap()
            });
            handlers.push(handler);
        }
        let mut local_predictions = Vec::with_capacity(MAX_THREADS);
        let mut batch_load_times = Vec::with_capacity(MAX_THREADS);
        let mut batch_preprocess_times = Vec::with_capacity(MAX_THREADS); 
        let mut batch_query_times = Vec::with_capacity(MAX_THREADS);
        let mut select_prediction_times = Vec::with_capacity(MAX_THREADS);
        for handler in handlers {
            let local = handler.join().unwrap();
            let (local_prediction, batch_load_time, batch_preprocess_time, batch_query_time, select_prediction_time) = local;
            local_predictions.push(local_prediction);
            batch_load_times.push(batch_load_time);
            batch_preprocess_times.push(batch_preprocess_time);
            batch_query_times.push(batch_query_time);
            select_prediction_times.push(select_prediction_time);
        }
        (local_predictions, batch_load_times, batch_preprocess_times, batch_query_times, select_prediction_times)
    }).unwrap();
    let elapsed_thread = start_thread.elapsed().as_millis();
    (predictions, load_time, preprocess_time, query_time, prediction_time, elapsed_thread)
}

fn k_nearest_neighbors_multithread_with_arc(strategy: usize, k: usize, n_neighbors:usize, f_trains: &[String], f_tests: &[String], n_line_trains: &[usize], n_line_tests: &[usize], n_batch: usize) -> (Vec<Vec<Arc<String>>>, Vec<u128>, Vec<u128>, Vec<u128>, Vec<u128>, u128){
    let start_thread = Instant::now();
    let (predictions, load_time, preprocess_time, query_time, prediction_time) = crossbeam::scope(|scope| {
        let mut handlers = Vec::with_capacity(MAX_THREADS);
        for i in 0..MAX_THREADS {
            let handler = scope.spawn(move |_| {
                k_nearest_neighbors_with_arc(strategy, k, n_neighbors, &f_trains[i], &f_tests[i], n_line_trains[i], n_line_tests[i], n_batch).unwrap()
            });
            handlers.push(handler);
        }
        let mut local_predictions = Vec::with_capacity(MAX_THREADS);
        let mut batch_load_times = Vec::with_capacity(MAX_THREADS);
        let mut batch_preprocess_times = Vec::with_capacity(MAX_THREADS); 
        let mut batch_query_times = Vec::with_capacity(MAX_THREADS);
        let mut select_prediction_times = Vec::with_capacity(MAX_THREADS);
        for handler in handlers {
            let local = handler.join().unwrap();
            let (local_prediction, batch_load_time, batch_preprocess_time, batch_query_time, select_prediction_time) = local;
            local_predictions.push(local_prediction);
            batch_load_times.push(batch_load_time);
            batch_preprocess_times.push(batch_preprocess_time);
            batch_query_times.push(batch_query_time);
            select_prediction_times.push(select_prediction_time);
        }
        (local_predictions, batch_load_times, batch_preprocess_times, batch_query_times, select_prediction_times)
    }).unwrap();
    let elapsed_thread = start_thread.elapsed().as_millis();
    (predictions, load_time, preprocess_time, query_time, prediction_time, elapsed_thread)
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

fn k_nearest_neighbors(strategy: usize, k: usize, n_neighbors:usize, f_train: &str, f_test: &str, n_line_train: usize, n_line_test: usize, n_batch: usize) -> Result<(Vec<String>, u128, u128, u128 , u128), &'static str> {
    match strategy {
        1 => Ok(k_nearest_neighbors_keep(k, n_neighbors, f_train, f_test, n_line_train, n_line_test, n_batch)), 
        2 => Ok(k_nearest_neighbors_dealloc(k, n_neighbors, f_train, f_test, n_line_train, n_line_test, n_batch)), 
        _ => Err("Wrong strategy"), 
    }
}

fn k_nearest_neighbors_with_arc(strategy: usize, k: usize, n_neighbors:usize, f_train: &str, f_test: &str, n_line_train: usize, n_line_test: usize, n_batch: usize) -> Result<(Vec<Arc<String>>, u128, u128, u128 , u128), &'static str> {
    match strategy {
        1 => Ok(k_nearest_neighbors_with_arc_keep(k, n_neighbors, f_train, f_test, n_line_train, n_line_test, n_batch)), 
        2 => Ok(k_nearest_neighbors_with_arc_dealloc(k, n_neighbors, f_train, f_test, n_line_train, n_line_test, n_batch)), 
        _ => Err("Wrong strategy"), 
    }
}

fn k_nearest_neighbors_keep(k: usize, n_neighbors:usize, f_train: &str, f_test: &str, n_line_train: usize, n_line_test: usize, n_batch: usize) -> (Vec<String>, u128, u128, u128 , u128) {

    let batch_size_trains = get_batch_size(n_line_train, n_batch);
    let batch_size_tests = get_batch_size(n_line_test, n_batch);
    let mut prediction = Vec::with_capacity(n_batch);

    let mut load_time = 0;
    let mut preprocess_time = 0;
    let mut query_time = 0;
    let mut stack_time = 0;
    let mut combine_time = 0;

    // loading phase
    let start_load_time = Instant::now();
    let path_train = Path::new(&f_train);
    let path_test = Path::new(&f_test);
    let file_test  = File::open(path_test).unwrap();
    let buf_reader_test = BufReader::new(file_test);
    let mut lines_test = buf_reader_test.lines().peekable();
    let elapsed_load_time = start_load_time.elapsed().as_millis();
    load_time += elapsed_load_time;

    for i in 0..n_batch {

        let start_batch_load = Instant::now();
        let batch_test = read_n_lines(&mut lines_test, batch_size_tests[i]).unwrap();
        let mut all_similarities = Vec::with_capacity(n_batch);
        let mut all_ids = Vec::with_capacity(n_batch);
        let file_train  = File::open(path_train).unwrap();
        let buf_reader_train = BufReader::new(file_train);
        let mut lines_train = buf_reader_train.lines().peekable();
        let elapsed_batch_load = start_batch_load.elapsed().as_millis();
        load_time += elapsed_batch_load;

        for j in 0..n_batch {

            let start_additional_load = Instant::now();
            let batch_train = read_n_lines(&mut lines_train, batch_size_trains[j]).unwrap();
            let elapsed_addtional_load = start_additional_load.elapsed().as_millis();
            load_time += elapsed_addtional_load;

            // preprocess phase
            let start_batch_preproccess = Instant::now();
            let train_words = split_documents(&batch_train[..]);
            let test_words = split_documents(&batch_test[..]); 
            let top_k = feature_map(&train_words[..], k);
            let train = create_id_numeric(&train_words[..], &top_k);
            let test = create_id_numeric(&test_words[..], &top_k);
            let (x_source_train, y_source_train) = split_x_y(&train[..]);
            let (x_source_test, y_source_test) = split_x_y(&test[..]);
            drop(y_source_test);
            let x_train = vectorize_x(&x_source_train[..]);
            let x_test = vectorize_x(&x_source_test[..]);
            let (y_train, decode_map)= vectorize_y(&y_source_train[..]);
            let elapsed_batch_preprocess = start_batch_preproccess.elapsed().as_millis();
            preprocess_time += elapsed_batch_preprocess;

            // query phase
            let start_batch_query = Instant::now();
            let (similarities, labels) = knn(&x_train, &y_train, &x_test, n_neighbors);
            let elapsed_batch_query = start_batch_query.elapsed().as_millis();
            query_time += elapsed_batch_query;

            // stack phase 
            let start_batch_stack = Instant::now();
            all_similarities.push(similarities);
            let ids = get_ids_from_labels(&labels, &decode_map);
            all_ids.push(ids);
            let elapsed_batch_stack = start_batch_stack.elapsed().as_millis();
            stack_time += elapsed_batch_stack;
        }
        // combine phase
        let start_batch_combine = Instant::now(); 
        let (combined_similarity, combined_id) = combine_neighbors(all_similarities, all_ids);
        let (combined_label, combined_decode_map) = vectorize_ids(&combined_id[..]);
        let (n_similarities, n_labels) = select_neighbors_combine(&combined_similarity, &combined_label, n_neighbors);
        drop(n_similarities);
        let label = select_neighbor(&n_labels);
        let mut id = get_id_from_label(&label, &combined_decode_map);
        prediction.append(&mut id);
        let elapsed_batch_combine = start_batch_combine.elapsed().as_millis();
        combine_time += elapsed_batch_combine;

    }
    (prediction, load_time, preprocess_time, query_time, stack_time + combine_time)
}

fn k_nearest_neighbors_dealloc(k: usize, n_neighbors:usize, f_train: &str, f_test: &str, n_line_train: usize, n_line_test: usize, n_batch: usize) -> (Vec<String>, u128, u128, u128 , u128) {
    
    
    let batch_size_trains = get_batch_size(n_line_train, n_batch);
    let batch_size_tests = get_batch_size(n_line_test, n_batch);
    let mut prediction = Vec::with_capacity(n_batch);

    let mut load_time = 0;
    let mut preprocess_time = 0;
    let mut query_time = 0;
    let mut stack_time = 0;
    let mut combine_time = 0;

    // loading phase
    let start_load_time = Instant::now();
    let path_train = Path::new(&f_train);
    let path_test = Path::new(&f_test);
    let file_test  = File::open(path_test).unwrap();
    let buf_reader_test = BufReader::new(file_test);
    let mut lines_test = buf_reader_test.lines().peekable();
    let elapsed_load_time = start_load_time.elapsed().as_millis();
    load_time += elapsed_load_time;

    for i in 0..n_batch {

        let start_batch_load = Instant::now();
        let batch_test = read_n_lines(&mut lines_test, batch_size_tests[i]).unwrap();
        let mut all_similarities = Vec::with_capacity(n_batch);
        let mut all_ids = Vec::with_capacity(n_batch);
        let file_train  = File::open(path_train).unwrap();
        let buf_reader_train = BufReader::new(file_train);
        let mut lines_train = buf_reader_train.lines().peekable();
        let elapsed_batch_load = start_batch_load.elapsed().as_millis();
        load_time += elapsed_batch_load;

        for j in 0..n_batch {

            let start_additional_load = Instant::now();
            let batch_train = read_n_lines(&mut lines_train, batch_size_trains[j]).unwrap();
            let elapsed_addtional_load = start_additional_load.elapsed().as_millis();
            load_time += elapsed_addtional_load;

            // preprocess phase
            let start_batch_preproccess = Instant::now();
            let train_words = split_documents(&batch_train[..]);
            drop(batch_train);
            let test_words = split_documents(&batch_test[..]); 
            let top_k = feature_map(&train_words[..], k);
            let train = create_id_numeric(&train_words[..], &top_k);
            drop(train_words);
            let test = create_id_numeric(&test_words[..], &top_k);
            drop(test_words);
            drop(top_k);
            let (x_source_train, y_source_train) = split_x_y(&train[..]);
            drop(train);
            let (x_source_test, y_source_test) = split_x_y(&test[..]);
            drop(test);
            drop(y_source_test);
            let x_train = vectorize_x(&x_source_train[..]);
            drop(x_source_train);
            let x_test = vectorize_x(&x_source_test[..]);
            drop(x_source_test); 
            let (y_train, decode_map)= vectorize_y(&y_source_train[..]);
            drop(y_source_train); 
            let elapsed_batch_preprocess = start_batch_preproccess.elapsed().as_millis();
            preprocess_time += elapsed_batch_preprocess;

            // query phase
            let start_batch_query = Instant::now();
            let (similarities, labels) = knn(&x_train, &y_train, &x_test, n_neighbors);
            drop(x_train);
            drop(x_test);
            drop(y_train);
            let elapsed_batch_query = start_batch_query.elapsed().as_millis();
            query_time += elapsed_batch_query;

            // stack phase 
            let start_batch_stack = Instant::now();
            all_similarities.push(similarities);
            let ids = get_ids_from_labels(&labels, &decode_map);
            all_ids.push(ids);
            let elapsed_batch_stack = start_batch_stack.elapsed().as_millis();
            stack_time += elapsed_batch_stack;
        }
        // combine phase
        let start_batch_combine = Instant::now(); 
        let (combined_similarity, combined_id) = combine_neighbors(all_similarities, all_ids);
        let (combined_label, combined_decode_map) = vectorize_ids(&combined_id[..]);
        drop(combined_id);
        let (n_similarities, n_labels) = select_neighbors_combine(&combined_similarity, &combined_label, n_neighbors);
        drop(n_similarities);
        drop(combined_similarity);
        drop(combined_label);
        let label = select_neighbor(&n_labels);
        drop(n_labels);
        let mut id = get_id_from_label(&label, &combined_decode_map);
        prediction.append(&mut id);
        let elapsed_batch_combine = start_batch_combine.elapsed().as_millis();
        combine_time += elapsed_batch_combine;

    }
    (prediction, load_time, preprocess_time, query_time, stack_time + combine_time)
}



fn k_nearest_neighbors_with_arc_keep(k: usize, n_neighbors:usize, f_train: &str, f_test: &str, n_line_train: usize, n_line_test: usize, n_batch: usize) -> (Vec<Arc<String>>, u128, u128, u128 , u128) {

    let batch_size_trains = get_batch_size(n_line_train, n_batch);
    let batch_size_tests = get_batch_size(n_line_test, n_batch);
    let mut prediction = Vec::with_capacity(n_batch);

    let mut load_time = 0;
    let mut preprocess_time = 0;
    let mut query_time = 0;
    let mut stack_time = 0;
    let mut combine_time = 0;

    // loading phase
    let start_load_time = Instant::now();
    let path_train = Path::new(&f_train);
    let path_test = Path::new(&f_test);
    let file_test  = File::open(path_test).unwrap();
    let buf_reader_test = BufReader::new(file_test);
    let mut lines_test = buf_reader_test.lines().peekable();
    let elapsed_load_time = start_load_time.elapsed().as_millis();
    load_time += elapsed_load_time;

    for i in 0..n_batch {

        let start_batch_load = Instant::now();
        let batch_test = read_n_lines(&mut lines_test, batch_size_tests[i]).unwrap();
        let mut all_similarities = Vec::with_capacity(n_batch);
        let mut all_ids = Vec::with_capacity(n_batch);
        let file_train  = File::open(path_train).unwrap();
        let buf_reader_train = BufReader::new(file_train);
        let mut lines_train = buf_reader_train.lines().peekable();
        let elapsed_batch_load = start_batch_load.elapsed().as_millis();
        load_time += elapsed_batch_load;

        for j in 0..n_batch {

            let start_additional_load = Instant::now();
            let batch_train = read_n_lines(&mut lines_train, batch_size_trains[j]).unwrap();
            let elapsed_addtional_load = start_additional_load.elapsed().as_millis();
            load_time += elapsed_addtional_load;

            // preprocess phase
            let start_batch_preproccess = Instant::now();
            let train_words = split_documents_with_arc(&batch_train[..]);
            let test_words = split_documents_with_arc(&batch_test[..]); 
            let top_k = feature_map_with_arc(&train_words[..], k);
            let train = create_id_numeric_with_arc(&train_words[..], &top_k);
            let test = create_id_numeric_with_arc(&test_words[..], &top_k);
            let (x_source_train, y_source_train) = split_x_y_with_arc(&train[..]);
            let (x_source_test, y_source_test) = split_x_y_with_arc(&test[..]);
            drop(y_source_test);
            let x_train = vectorize_x_with_arc(&x_source_train[..]);
            let x_test = vectorize_x_with_arc(&x_source_test[..]);
            let (y_train, decode_map)= vectorize_y_with_arc(&y_source_train[..]);
            let elapsed_batch_preprocess = start_batch_preproccess.elapsed().as_millis();
            preprocess_time += elapsed_batch_preprocess;

            // query phase
            let start_batch_query = Instant::now();
            let (similarities, labels) = knn(&x_train, &y_train, &x_test, n_neighbors);
            let elapsed_batch_query = start_batch_query.elapsed().as_millis();
            query_time += elapsed_batch_query;

            // stack phase 
            let start_batch_stack = Instant::now();
            all_similarities.push(similarities);
            let ids = get_ids_from_labels_with_arc(&labels, &decode_map);
            all_ids.push(ids);
            let elapsed_batch_stack = start_batch_stack.elapsed().as_millis();
            stack_time += elapsed_batch_stack;
        }
        // combine phase
        let start_batch_combine = Instant::now(); 
        let (combined_similarity, combined_id) = combine_neighbors_with_arc(all_similarities, all_ids);
        let (combined_label, combined_decode_map) = vectorize_ids_with_arc(&combined_id[..]);
        let (n_similarities, n_labels) = select_neighbors_combine(&combined_similarity, &combined_label, n_neighbors);
        drop(n_similarities);
        let label = select_neighbor(&n_labels);
        let mut id = get_id_from_label_with_arc(&label, &combined_decode_map);
        prediction.append(&mut id);
        let elapsed_batch_combine = start_batch_combine.elapsed().as_millis();
        combine_time += elapsed_batch_combine;

    }
    (prediction, load_time, preprocess_time, query_time, stack_time + combine_time)
}

fn k_nearest_neighbors_with_arc_dealloc(k: usize, n_neighbors:usize, f_train: &str, f_test: &str, n_line_train: usize, n_line_test: usize, n_batch: usize) -> (Vec<Arc<String>>, u128, u128, u128 , u128) {

    let batch_size_trains = get_batch_size(n_line_train, n_batch);
    let batch_size_tests = get_batch_size(n_line_test, n_batch);
    let mut prediction = Vec::with_capacity(n_batch);

    let mut load_time = 0;
    let mut preprocess_time = 0;
    let mut query_time = 0;
    let mut stack_time = 0;
    let mut combine_time = 0;

    // loading phase
    let start_load_time = Instant::now();
    let path_train = Path::new(&f_train);
    let path_test = Path::new(&f_test);
    let file_test  = File::open(path_test).unwrap();
    let buf_reader_test = BufReader::new(file_test);
    let mut lines_test = buf_reader_test.lines().peekable();
    let elapsed_load_time = start_load_time.elapsed().as_millis();
    load_time += elapsed_load_time;

    for i in 0..n_batch {

        let start_batch_load = Instant::now();
        let batch_test = read_n_lines(&mut lines_test, batch_size_tests[i]).unwrap();
        let mut all_similarities = Vec::with_capacity(n_batch);
        let mut all_ids = Vec::with_capacity(n_batch);
        let file_train  = File::open(path_train).unwrap();
        let buf_reader_train = BufReader::new(file_train);
        let mut lines_train = buf_reader_train.lines().peekable();
        let elapsed_batch_load = start_batch_load.elapsed().as_millis();
        load_time += elapsed_batch_load;

        for j in 0..n_batch {

            let start_additional_load = Instant::now();
            let batch_train = read_n_lines(&mut lines_train, batch_size_trains[j]).unwrap();
            let elapsed_addtional_load = start_additional_load.elapsed().as_millis();
            load_time += elapsed_addtional_load;

            // preprocess phase
            let start_batch_preproccess = Instant::now();
            let train_words = split_documents_with_arc(&batch_train[..]);
            drop(batch_train);
            let test_words = split_documents_with_arc(&batch_test[..]); 
            let top_k = feature_map_with_arc(&train_words[..], k);
            let train = create_id_numeric_with_arc(&train_words[..], &top_k);
            drop(train_words);
            let test = create_id_numeric_with_arc(&test_words[..], &top_k);
            drop(test_words);
            drop(top_k);
            let (x_source_train, y_source_train) = split_x_y_with_arc(&train[..]);
            drop(train);
            let (x_source_test, y_source_test) = split_x_y_with_arc(&test[..]);
            drop(test);
            drop(y_source_test);
            let x_train = vectorize_x_with_arc(&x_source_train[..]);
            drop(x_source_train);
            let x_test = vectorize_x_with_arc(&x_source_test[..]);
            drop(x_source_test); 
            let (y_train, decode_map)= vectorize_y_with_arc(&y_source_train[..]);
            drop(y_source_train); 
            let elapsed_batch_preprocess = start_batch_preproccess.elapsed().as_millis();
            preprocess_time += elapsed_batch_preprocess;

            // query phase
            let start_batch_query = Instant::now();
            let (similarities, labels) = knn(&x_train, &y_train, &x_test, n_neighbors);
            drop(x_train);
            drop(x_test);
            drop(y_train);
            let elapsed_batch_query = start_batch_query.elapsed().as_millis();
            query_time += elapsed_batch_query;

            // stack phase 
            let start_batch_stack = Instant::now();
            all_similarities.push(similarities);
            let ids = get_ids_from_labels_with_arc(&labels, &decode_map);
            all_ids.push(ids);
            let elapsed_batch_stack = start_batch_stack.elapsed().as_millis();
            stack_time += elapsed_batch_stack;
        }
        // combine phase
        let start_batch_combine = Instant::now(); 
        let (combined_similarity, combined_id) = combine_neighbors_with_arc(all_similarities, all_ids);
        let (combined_label, combined_decode_map) = vectorize_ids_with_arc(&combined_id[..]);
        drop(combined_id);
        let (n_similarities, n_labels) = select_neighbors_combine(&combined_similarity, &combined_label, n_neighbors);
        drop(n_similarities);
        drop(combined_similarity);
        drop(combined_label);
        let label = select_neighbor(&n_labels);
        drop(n_labels);
        let mut id = get_id_from_label_with_arc(&label, &combined_decode_map);
        prediction.append(&mut id);
        let elapsed_batch_combine = start_batch_combine.elapsed().as_millis();
        combine_time += elapsed_batch_combine;

    }
    (prediction, load_time, preprocess_time, query_time, stack_time + combine_time)
}

// fn drop_dump_and_time<T>(data: T, object_name: &str, thread: usize, batch: usize) -> u128 
//     where T: Serialize 
// {
//     let t = dump_and_time(&data, object_name, thread, batch);
//     drop(data);
//     t
// }

// fn dump_and_time<T>(data: &T, object_name: &str, thread: usize, batch: usize) -> u128
//     where T: Serialize 
// {
//     let start = Instant::now();
//     dump_to_disk(data, object_name, thread, batch);
//     let elapsed = start.elapsed().as_millis();
//     elapsed
// }

// fn dump_to_disk<T>(data: &T, object_name: &str, thread: usize, batch: usize)
// where T: Serialize, 
// {
//     let serialized = serde_json::to_string(&data).unwrap();
//     let mut file = OpenOptions::new()
//             .write(true)
//             .create(true)
//             .open(format!("serialized/{}_{}_{}", object_name, thread, batch))
//             .unwrap();
//     file.write_all(serialized.as_bytes()).expect("Fail to write file.");
// }

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

