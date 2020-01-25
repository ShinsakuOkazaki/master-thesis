extern crate rand;
extern crate bytes;
//use std::time::Instant;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;
use rand::distributions::{Distribution, Uniform};
//use bytes::{Bytes, BytesMut, Buf, BufMut};

fn main() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let in_string = get_random_string();
    let in_buf = in_string.as_bytes();
    {
        let mut file = File::create("buffer.txt")?;
        file.write(in_buf)?;
    }
    let file = File::open("buffer.txt")?;
    let mut reader = BufReader::new(file);
    let out_buf = reader.fill_buf()?;
    let length = out_buf.len();
    stdin.consume(length);
    assert_eq!(in_buf, out_buf);
    let out_string = String::from_utf8_lossy(out_buf);
    assert_eq!(in_string, out_string);
    println!("in_string: {}, out_string: {}", in_string, out_string);
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