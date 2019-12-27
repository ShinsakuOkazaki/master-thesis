use std::env;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::prelude::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let size: usize = args[1].parse().unwrap();
    let initialization: bool = args[2].parse().unwrap();
    vector_addition(size, initialization)
}


fn vector_addition(size: usize, initialization: bool) {
    match size {
        1 => add_to_vector(10, initialization), 
        2 => add_to_vector(100, initialization), 
        3 => add_to_vector(1000, initialization), 
        4 => add_to_vector(10000, initialization), 
        _ => println!("Invalid input!"),
    }
}

fn add_to_vector(size: usize, initialization: bool) {
    
    println!("size: {}, initialization: {}", size, initialization);
    let start_init = Instant::now();
    let mut vector;
    if initialization {
        vector = Vec::with_capacity(size);
    } else {
        vector = Vec::new();
    }

    let elapsed_init = start_init.elapsed().as_nanos();

    let mut char_arr_vec = Vec::new();
    for _i in 0..size {
        let char_arr = ['a', 'b', 'c', 'd', 'e', 
                        'f', 'g', 'h', 'i', 'j'];
        char_arr_vec.push(char_arr);
    }
    
    let start_add = Instant::now();
    
    for _i in 0..size {
        vector.push(char_arr_vec.pop());
    }

    let elapsed_add = start_add.elapsed().as_nanos();
    let elapsed_total = start_init.elapsed().as_nanos();

    let output = format!("[CharArray]#{:?}#{:?}#{:?}#{:?}#{:?}\n", initialization, size, elapsed_init, elapsed_add, elapsed_total);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");
}
