use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use regex::Regex;
use std::collections::HashSet;
use stopwords::{Spark, Language, Stopwords};
use ndarray::{Array, Ix1, Ix2, stack, Axis};

pub fn vectorize_ids(ids: &[Vec<String>]) -> (Array<i32, Ix2>, HashMap<i32, String>) {
    let n = ids.len();
    let m = ids[0].len();
    let mut encode_map = HashMap::new();
    let mut decode_map = HashMap::new();
    let mut encode = 0;
    for i in 0..n {
        for j in 0..m {
            if !encode_map.contains_key(&ids[i][j]) {
                encode_map.insert(ids[i][j].clone(), encode);
                decode_map.insert(encode, ids[i][j].clone());
                encode += 1;
            }
        }
    }
    let mut vectorized = Array::zeros((n, m));
    for i in 0..n {
        for j in 0..m {
            encode = *encode_map.get(&ids[i][j]).unwrap();
            vectorized[[i, j]] = encode;
        }
    }
    (vectorized, decode_map)
}


pub fn combine_neighbors(similarities: Vec<Array<f64, Ix2>>, mut ids: Vec<Vec<Vec<String>>>) -> (Array<f64, Ix2>, Vec<Vec<String>>){
     let (n, m) = similarities[0].dim();
     let l = ids.len();
     let o = ids[0].len();
     let mut combined_x = similarities[0].clone(); 
     let mut combined_y = ids[0].clone();
     for i in 1..l {
        combined_x = stack![Axis(1), combined_x, similarities[i]];
        concat_string_vectors(&mut combined_y[..], ids[i].clone());
     }
     (combined_x, combined_y)
}


pub fn concat_string_vectors(source: &mut [Vec<String>], mut other: Vec<Vec<String>>) {
    let n = source.len();
    for i in 0..n {
        source[i].append(&mut other[i]);
    }
}

pub fn get_id_from_label(label: &Array<i32, Ix1>, decode_map: &HashMap<i32, String>) -> Vec<String> {
    let n= label.dim();
    let mut res = Vec::with_capacity(n);
    for i in 0..n {
        let id = decode_map.get(&label[i]).unwrap();
        res.push(id.clone());
    }
    res
}

pub fn get_ids_from_labels(labels: &Array<i32, Ix2>, decode_map: &HashMap<i32, String>) -> Vec<Vec<String>> {
    let (n, m) = labels.dim();
    let mut res = Vec::with_capacity(n);
    for i in 0..n {
        let mut v = Vec::with_capacity(m);
        for j in 0..m {
            let id = decode_map.get(&labels[[i, j]]).unwrap();
            v.push(id.clone());
        }
        res.push(v);
    }
    res
}

pub fn split_x_y(source: &[(String, Vec<f64>)]) -> (Vec<Vec<f64>>, Vec<String>) {
    let n = source.len();
    let mut res_x = Vec::with_capacity(n);
    let mut res_y = Vec::with_capacity(n);
    for i in 0..n {
        res_x.push(source[i].1.clone());
        res_y.push(source[i].0.clone());
    }
    (res_x, res_y)
}

pub fn vectorize_x(source: &[Vec<f64>]) -> Array<f64, Ix2>{

    let n = source.len();
    let m = source[0].len();
    let mut vector = Vec::with_capacity(n * m);
    for i in 0..n {
        vector.extend_from_slice(&source[i][..]);
    }
    
    let x = Array::from_shape_vec((n, m), vector).unwrap();
    x
}

pub fn vectorize_y(source: &[String]) -> (Array<i32, Ix1>, HashMap<i32, String>) {
    let n = source.len();
    let mut encode_map = HashMap::new();
    let mut decode_map = HashMap::new();
    let mut id1;
    let mut id2;
    let mut encode = 0;
    for i in 0..n {
        id1 = source[i].clone();
        id2 = source[i].clone(); 
        if !encode_map.contains_key(&id1) {
            encode_map.insert(id1, encode);
            decode_map.insert(encode, id2);
            encode += 1;
        }
    }

    let mut y = Array::zeros(n);
    for i in 0..n {
        encode = *encode_map.get(&source[i]).unwrap();
        y[i] = encode;
    }
    (y, decode_map)
}



pub fn split_documents(batch: &[String]) -> Vec<(String, Vec<String>)> {
    let filtered = filter(&batch[..]);
    let id_sentence = create_id_sentence(&filtered[..]); 
    let id_words = create_wordlist(&id_sentence[..]);
    return id_words;
}

pub fn feature_map(id_words: &[(String, Vec<String>)], k: usize) -> HashMap<String, usize> {
    let count_map = create_count_map(id_words);
    let top_k = top(&count_map, k);
    return top_k;
}

// create id and numeric vector pair [("id1", [3, 1, ,5....]), ("id2", [1, 21,4 ,....]), ....]
pub fn create_id_numeric(arr: &[(String, Vec<String>)], map: &HashMap<String, usize>) -> Vec<(String, Vec<f64>)>{
    let n = arr.len();
    let mut res = Vec::with_capacity(n);
    let mut numeric;
    let mut pair;
    for (id, words) in arr {
        numeric = create_numeric_vector(words, &map);
        pair = (id.clone(), numeric);
        res.push(pair);
    }
    res
}

// create numeric vector from vector of words ["safa", "afeaa", ....] -> [3, 1, 2 .....]
fn create_numeric_vector(words: &[String], map: &HashMap<String, usize>) -> Vec<f64> {
    let n = words.len();
    let k = map.len();
    let mut res = vec![0.0; k];
    let mut key;
    let mut idx;
    for i in 0..n {
        key = &words[i];
        idx = map.get(key);
        match idx {
            Some(x) => res[*x] += 1.0, 
            None => ()
        }
    }
    res
}

// Function to read file and give vecto ["I am shinsaku", "You are shinsaku"].
fn read_file(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let buf_reader = BufReader::new(file);
    let mut arr = Vec::new();
    for line in buf_reader.lines() {
        arr.push(line.unwrap());
    }
    Ok(arr)
}

// Function to filter out line that does not have id and url.
fn filter(arr: &[String]) -> Vec<String>{
    let mut res = Vec::new();
    for line in arr {
        if line.contains("id") && line.contains("url=") {
            res.push(line.clone());
        }
    }
    res
}


// create [("id1", "sentence1"), ("id2", "sentence2"), .....]
fn create_id_sentence(arr: &[String]) -> Vec<(String, String)> {
    let n = arr.len();
    let mut res = Vec::with_capacity(n);
    let mut id;
    let mut sentence;
    let mut pair;
    for line in arr {
        id = String::from(&line[(line.find(r#"id=""#).unwrap() + 4)..line.find(r#"" url="#).unwrap()]);
        sentence = String::from(&line[line.find(r#"">"#).unwrap() + 2..line.len() - 6]);
        pair = (id, sentence);
        res.push(pair);
    }
    res
}

// convert [("id1", "sentence1"), ("id2", "sentence2"), .....] -> [("id1", ["sen", tence1"), ("id2", "sentence2"), .....] 
fn create_wordlist(arr: &[(String, String)]) -> Vec<(String, Vec<String>)> {
    let n = arr.len();
    let mut res = Vec::with_capacity(n);
    let re = Regex::new(r"[^a-zA-Z]").unwrap();
    let mut words: Vec<&str>;
    let mut converted: Vec<String>;
    let mut pair;
    let stops: HashSet<_> = Spark::stopwords(Language::English).unwrap().iter().collect();
    for p in arr {
        words = re.split(&p.1[..]).collect();
        converted = create_words_lowercase(&words[..], &stops);
        pair = (p.0.clone(), converted);
        res.push(pair);
    }
    res
}

// convert ["Word1",  "WOrd2", ...] -> ["word1", "word2", ....]
fn create_words_lowercase(words: &[&str], stops: &HashSet<&&str>) -> Vec<String>{
    let n = words.len();
    let mut res = Vec::with_capacity(n);
    let mut lower_word;
    for i in 0..n {
        lower_word = create_string_lowercase(&words[i]);
        if lower_word.len() != 0  && !check_stopword(&lower_word[..], stops){
            res.push(lower_word);
        }        
    }
    res
}

fn check_stopword(word: &str, stops: &HashSet<&&str>) -> bool{
    let res = stops.contains(&word);
    res
}

// convert "Word" to "word"
fn create_string_lowercase(word: &str) -> String{
    let n = word.len();
    let mut res = String::with_capacity(n);
    for c in word.chars() {
        if c.is_ascii_uppercase() {
            res.push(c.to_ascii_lowercase());
        } else {
            res.push(c);
        }
    }
    res
}


// create from [("id1", ["st", "sse", ....]), ("id1", ["ae", "aef", ...])...] -> {"word1": 323, "word2": 2939, .....}
fn create_count_map(arr: &[(String, Vec<String>)]) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for (_, d) in arr {
        for i in 0..d.len() {
            let key = d[i].clone();
            let count = map.entry(key).or_insert(0);
            *count += 1;
        }        
    }
    map
}

// take topK word and create map with word to rank {"word": count1, "word2": count2, .....} -> {"word2": rank0, "word1", rank1....}
fn top(map: &HashMap<String, i32>, k: usize) ->  HashMap<String, usize>{
    let mut pq = PriorityQueue::with_capacity(k);
    let mut res = HashMap::with_capacity(k);
    for (s, v) in map {
        pq.push(s, Reverse(v));
        if pq.len() > k {
            pq.pop();
        }
    }
    let mut rank: usize = 0;
    for (s, _v) in pq.into_sorted_iter() {
        res.insert(s.clone(), rank);
        rank += 1;
    }
    res
}


