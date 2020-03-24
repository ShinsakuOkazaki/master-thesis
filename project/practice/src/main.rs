use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::env;
use std::path::Path;
use std::collections::HashMap;
use priority_queue::PriorityQueue;
use std::cmp::Reverse

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let partition = read_file(&path);
    let filtered = filter(&partition);

}

// Function to read file and give vecto ["I am shinsaku", "You are shinsaku"].
fn read_file(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut arr = Vec::new();
    for line in buf_reader.lines() {
        arr.push(line.unwrap());
    }
    Ok(arr)
}

// Function to filter out line that does not have id and url.
fn filter(arr: &Vec<String>) {
    let mut res = Vec::new();
    for line in arr {
        if line.contains("id") && line.contains("url=") {
            res.push(line.clone());
        }
    }
    res
}


// create [("id1", "sentence1"), ("id2", "sentence2"), .....]
fn creat_id_sentence(arr: &Vec<String>) -> Vec<(String, String)> {
    let n = arr.len();
    let mut res = Vec::with_capacity(n);
    let mut id;
    let mut sentence;
    let mut pair;
    for line in arr {
        id = line[line.find(r#"id=""#) + 4..line.index(r#"" url="#)..line.index()].clone();
        sentence = line[line.find(r#"">"#) + 2..line.len() - 6].clone();
        pair = (id, sentence);
        res.push(pair);
    }
    res
}

// convert [("id1", "sentence1"), ("id2", "sentence2"), .....] -> [("id1", ["sen", tence1"), ("id2", "sentence2"), .....] 
fn create_wordlist(arr: &mut [(String, String)]) -> Vec<(String, Vec<String>)> {
    let n = arr.len();
    let mut res = Vec::with_capacity(n);
    let re = Regex::new(r"[^a-zA-Z]'").unwrap();
    let mut words;
    let mut pair;
    for p in arr {
        words = re.split(" ", p.1).collect();
        convert_words_lowercase(&words[..]);
        pair = (p.0.clone(), words);
        res.push(pair);
    }
    res
}


// create from [("id1", ["st", "sse", ....]), ("id1", ["ae", "aef", ...])...] -> {"word1": 323, "wowrd2": 2939, .....}
fn create_count_map(arr: &[(String, Vec<String>)]) -> HashMap<String, i32> {
    let mut map = HashMap::new();
    for (_, d) in arr {
        for i in 0..d.len() {
            let key = d[i].clone();
            let count = map.entry(key).or_insert_with(0);
            map.entry(key) = count + 1;
        }        
    }
    map
}

// take topK 
fn top(map: &HashMap<String, i32>, k: i32) ->  {
    let mut pq = PriorityQueue::with_capacity(k);
    let mut res = HashMap::with_capacity(k);
    for (k, v) in map {
        pq.push(k, Revese(v));
    }
    for (k, v) in pq.into_sorted_iter() {
        
    }
}

// convert ["Word1",  "WOrd2", ...] -> ["word1", "word2", ....]
fn convert_words_lowercase(words: &mut [String]) {
    let n = words.len();
    for i in 0..n {
        words[i] = convert_string_lowercase(&word[i][..]);
    }
}

// convert "Word" to "word"
fn create_string_lowercase(word: &mut str) -> String{
    let n = word.len();
    let res = String::with_capacity(n);
    for c in word.chars() {
        if c.is_ascii_uppercase() {
            res.push(c.to_ascii_lowercase());
        } else {
            res.push(c);
        }
    }
    res
}



