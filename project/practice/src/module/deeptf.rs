use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::path::Path;
use std::collections::HashMap;
use priority_queue::PriorityQueue;
use std::cmp::Reverse;
use regex::Regex;

pub fn split_documents(path: &Path) -> Vec<(String, Vec<String>)> {
    let partition = read_file(&path).unwrap();
    let filtered = filter(&partition[..]);
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
    for p in arr {
        words = re.split(&p.1[..]).collect();
        converted = create_words_lowercase(&words[..]);
        pair = (p.0.clone(), converted);
        res.push(pair);
    }
    res
}

// convert ["Word1",  "WOrd2", ...] -> ["word1", "word2", ....]
fn create_words_lowercase(words: &[&str]) -> Vec<String>{
    let n = words.len();
    let mut res = Vec::with_capacity(n);
    let mut lower_word;
    for i in 0..n {
        lower_word = create_string_lowercase(&words[i]);
        if lower_word.len() != 0 {
            res.push(lower_word);
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


