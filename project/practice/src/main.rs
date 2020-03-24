use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let partition = read_file(&path);
    let filtered = filter(&partition);

}


fn read_file(path: &Path) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let mut buf_reader = BufReader::new(file);
    let mut arr = Vec::new();
    for line in buf_reader.lines() {
        arr.push(line.unwrap());
    }
    Ok(arr)
}

fn filter(arr: &Vec<String>) {
    let mut res = Vec::new();
    for line in arr {
        if line.contains("id") && line.contains("url=") {
            res.push(line.clone());
        }
    }
    res
}

fn creat_id_sentence(arr: &Vec<String>) {
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

fn convert_t0_wordlist(arr: &mut [(String, String)]) {
    let n = arr.len();
    let re = Regex::new(r"[^a-zA-Z]'").unwrap();
    let mut words;
    for i in 0..n {
        words = re.split(" ", arr[i].1).collect();
        convert_words_lowercase(&words[..]);
        arr[i].1 = words;
    }
}

fn 

fn convert_words_lowercase(words: &mut [String]) {
    let n = words.len();
    for i in 0..n {
        words[i] = convert_string_lowercase(&word[i][..]);
    }
}

fn convert_string_lowercase(word: &mut str) {
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