extern crate rand;
extern crate bytes;
use std::env;
use std::time::Instant;
use std::io::prelude::*;
use std::ptr;
use std::mem;
use std::fs::File;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use rand::distributions::{Distribution, Uniform};
use bytes::{Bytes, BytesMut, Buf, BufMut};

fn main() {

    //let args: Vec<String> = env::args().collect();
    //let size: usize = args[1].parse().unwrap();
    let string = get_random_string();
    let byte_slice = string.as_bytes();
    write_bytes_tofile();
}

fn write_bytes_tofile(byte_slice: &[u8]) std::io::Result<()> {
    let mut file = File::create("buffer.txt")?;
    file.write(byte_slice)?;
    Ok(())
}


fn get_random_string() -> String {
    let dist = Uniform::from(3..7);
    let rand_string: String = thread_rng()
                            .sample_iter(&Alphanumeric)
                            .take(dist.sample(&mut thread_rng()))
                            .collect();
    return rand_string;
}