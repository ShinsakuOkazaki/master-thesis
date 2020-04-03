use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use regex::Regex;
use std::rc::Rc;
use ndarray::{Array,Ix1, Ix2};

pub fn get_id_from_label_with_rc(label: &Array<i32, Ix1>, decode_map: &HashMap<i32, Rc<String>>) -> Vec<Rc<String>> {
    let n= label.dim();
    let mut res = Vec::with_capacity(n);
    for i in 0..n {
        let id = decode_map.get(&label[i]).unwrap();
        res.push(Rc::clone(&id));
    }
    res
}


pub fn vectorize_x_with_rc(source: &[(Rc<String>, Vec<f64>)]) -> Array<f64, Ix2>{

    let n = source.len();
    let m = source[0].1.len();
    let mut vector = Vec::with_capacity(n * m);
    for i in 0..n {
        vector.extend_from_slice(&source[i].1[..]);
    }
    
    let x = Array::from_shape_vec((n, m), vector).unwrap();
    x
}

pub fn vectorize_y_with_rc(source: &[(Rc<String>, Vec<f64>)]) -> (Array<i32, Ix1>, HashMap<i32, Rc<String>>) {
    let n = source.len();
    let mut encode_map = HashMap::new();
    let mut decode_map = HashMap::new();
    let mut id1;
    let mut id2;
    let mut encode = 0;
    for i in 0..n {
        id1 = Rc::clone(&source[i].0);
        id2 = Rc::clone(&source[i].0); 
        if !encode_map.contains_key(&id1) {
            encode_map.insert(id1, encode);
            decode_map.insert(encode, id2);
            encode += 1;
        }
    }

    let mut y = Array::zeros(n);
    for i in 0..n {
        encode = *encode_map.get(&source[i].0).unwrap();
        y[i] = encode;
    }
    (y, decode_map)
}


pub fn split_documents_with_rc(path: &Path) -> Vec<(Rc<String>, Vec<Rc<String>>)> {
    let partition = read_file(&path).unwrap();
    let filtered = filter(&partition[..]);
    let id_sentence = create_id_sentence_with_rc(&filtered[..]); 
    let id_words = create_wordlist_with_rc(&id_sentence[..]);
    return id_words;
}

pub fn feature_map_with_rc(id_words: &[(Rc<String>, Vec<Rc<String>>)], k: usize) -> HashMap<Rc<String>, usize> {
    let count_map = create_count_map_with_rc(id_words);
    let top_k = top_with_rc(&count_map, k);
    return top_k;
}

//create id and numeric vector pair [("id1", [3, 1, ,5....]), ("id2", [1, 21,4 ,....]), ....]
pub fn create_id_numeric_with_rc(arr: &[(Rc<String>, Vec<Rc<String>>)], map: &HashMap<Rc<String>, usize>) -> Vec<(Rc<String>, Vec<f64>)>{
    let n = arr.len();
    let mut res = Vec::with_capacity(n);
    let mut numeric;
    let mut pair;
    for (id, words) in arr {
        numeric = create_numeric_vector_with_rc(words, &map);
        pair = (id.clone(), numeric);
        res.push(pair);
    }
    res
}

// create numeric vector from vector of words ["safa", "afeaa", ....] -> [3, 1, 2 .....]
fn create_numeric_vector_with_rc(words: &[Rc<String>], map: &HashMap<Rc<String>, usize>) -> Vec<f64> {
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
fn create_id_sentence_with_rc(arr: &[String]) -> Vec<(Rc<String>, String)> {
    let n = arr.len();
    let mut res = Vec::with_capacity(n);
    let mut id;
    let mut sentence;
    let mut pair;
    for line in arr {
        id = Rc::new(String::from(&line[(line.find(r#"id=""#).unwrap() + 4)..line.find(r#"" url="#).unwrap()]));
        sentence = String::from(&line[line.find(r#"">"#).unwrap() + 2..line.len() - 6]);
        pair = (id, sentence);
        res.push(pair);
    }
    res
}

// convert [("id1", "sentence1"), ("id2", "sentence2"), .....] -> [("id1", ["sen", tence1"), ("id2", "sentence2"), .....] 
fn create_wordlist_with_rc(arr: &[(Rc<String>, String)]) -> Vec<(Rc<String>, Vec<Rc<String>>)> {
    let n = arr.len();
    let mut res = Vec::with_capacity(n);
    let re = Regex::new(r"[^a-zA-Z]").unwrap();
    let mut words: Vec<&str>;
    let mut converted: Vec<Rc<String>>;
    let mut pair;
    for p in arr {
        words = re.split(&p.1[..]).collect();
        converted = create_words_lowercase_with_rc(&words[..]);
        pair = (Rc::clone(&p.0), converted);
        res.push(pair);
    }
    res
}

// convert ["Word1",  "WOrd2", ...] -> ["word1", "word2", ....]
fn create_words_lowercase_with_rc(words: &[&str]) -> Vec<Rc<String>>{
    let n = words.len();
    let mut res = Vec::with_capacity(n);
    let mut lower_word;
    for i in 0..n {
        lower_word = create_string_lowercase(&words[i]);
        if lower_word.len() != 0 {
            res.push(Rc::new(lower_word));
        }        
    }
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
fn create_count_map_with_rc(arr: &[(Rc<String>, Vec<Rc<String>>)]) -> HashMap<Rc<String>, i32> {
    let mut map = HashMap::new();
    for (_, d) in arr {
        for i in 0..d.len() {
            let key = Rc::clone(&d[i]);
            let count = map.entry(key).or_insert(0);
            *count += 1;
        }        
    }
    map
}

// take topK word and create map with word to rank {"word": count1, "word2": count2, .....} -> {"word2": rank0, "word1", rank1....}
fn top_with_rc(map: &HashMap<Rc<String>, i32>, k: usize) ->  HashMap<Rc<String>, usize>{
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
        res.insert(Rc::clone(&s), rank);
        rank += 1;
    }
    res
}

