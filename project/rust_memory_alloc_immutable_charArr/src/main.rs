extern crate rand;

use std::env;
use std::time::Instant;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::ptr;


fn main() {

    let args: Vec<String> = env::args().collect();
    
    let size: usize = args[1].parse().unwrap();
    add_to_imutable(size);

}

fn add_to_imutable(size: usize) {
    
    println!("size: {}", size);
    let start_init = Instant::now();
    let vector;
    vector = Vec::with_capacity(size);
    

    let elapsed_init = start_init.elapsed().as_nanos();

    let mut src_arr = Vec::with_capacity(size);
    for _i in 0..size {
        let char_arr = ['a', 'b', 'c', 'd', 'e', 
                        'f', 'g', 'h', 'i', 'j'];
        src_arr.push(char_arr);
    }
    
    let mut dst = vector;
    let src = &src_arr;
    let dst_ptr = dst.as_mut_ptr();
    let src_ptr = src.as_ptr();
    let start_add = Instant::now();
    unsafe {
        ptr::copy_nonoverlapping(src_ptr, dst_ptr, size);
    }

    let elapsed_add = start_add.elapsed().as_nanos();
    let elapsed_total = start_init.elapsed().as_nanos();

    let output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}\n", size, elapsed_init, elapsed_add, elapsed_total);

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");    
}
