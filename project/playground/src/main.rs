
extern crate rand;
use rand::thread_rng;
use rand::distributions::{Uniform, Distribution};
use std::time::Instant;
use std::io::prelude::*;
use std::env;
use std::fs::OpenOptions;
fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let method: i32 = args[2].parse().unwrap();
    run_experiment(size, method);
}

fn run_experiment(size: usize, method: i32) {
    match method {
        1 => run_ex_trait_object(size),
        2 => run_ex_generic_function(size),
        _ => println!("Wrong input")
    }
}

fn run_ex_trait_object(size: usize) {
    let shape_vector = get_shape_vector(size);
    let total = 0;
    let start = Instant::now();
    for i in 0..size {
        total += get_area_trait_object(&*shape_vector[i]);
    }
    let elapsed = start.elapsed().as_nanos();
    write_to_file(size, "trait", elapsed, total);
}

fn run_ex_generic_function(size: usize) {
    let shape_vector = get_shape_vector(size);
    let total = 0;
    let start = Instant::now();
    for i in 0..size {
        total += get_area_generic_function(&*shape_vector[i]);
    }
    let elapsed = start.elapsed().as_nanos();
    write_to_file(size, "generic", elapsed, total);
}

fn get_area_trait_object(shape: &dyn Shape) -> i32 {
    shape.get_area()
}

fn get_area_generic_function<T: Shape>(shape: &T) -> i32 {
    shape.get_area()
}

fn get_shape_vector(size: usize) -> Vec<Box<dyn Shape>> {
    let shape_vector = Vec::with_capacity(size);
    for _ in 0..size {
        let shape = get_random_shape();
        shape_vector.push(shape);
    }
    shape_vector
}

fn get_random_shape() -> Box<dyn Shape> {
    let dist = Uniform::from(1..4);
    let choice: i32 = dist.sample(&mut thread_rng()) as i32;
    match choice {
        1 => Box::new(get_triangle()),
        2 => Box::new(get_circle()),
        3 => Box::new(get_rectangle()),
    }
}

fn get_triangle() -> Triangle {
    Triangle {
        x: get_integer(),
        y: get_integer(),
        bottom: get_integer(),
        hight: get_integer(),
        angle: get_integer()
    }
}

fn get_circle() -> Circle {
    Circle {
        x: get_integer(),
        y: get_integer(),
        r_vertical: get_integer(),
        r_horizontal: get_integer(),
        angle: get_integer()
    }
}

fn get_rectangle() -> Rectangle {
    Rectangle {
        x: get_integer(),
        y: get_integer(),
        wide: get_integer(),
        hight: get_integer(),
        angle: get_integer()
    }
}

pub trait Shape {
    fn get_area(&self) -> i32;
}

pub struct Triangle {
    x: i32,
    y :i32,
    bottom: i32,
    hight: i32,
    angle: i32
}

pub struct Circle {
    x: i32,
    y: i32,
    r_vertical: i32,
    r_horizontal: i32,
    angle: i32
}

pub struct Rectangle {
    x: i32,
    y: i32,
    wide: i32,
    hight: i32,
    angle: i32
}

impl Shape for Triangle {
    fn get_area(&self) -> i32 {
        self.bottom * self.hight
    }
}

impl Shape for Circle {
    fn get_area(&self) -> i32 {
        3 * self.r_vertical * self.r_horizontal
    }
}

impl Shape for Rectangle {
    fn get_area(&self) -> i32 {
        self.wide * self.hight
    }
}


fn get_integer() -> i32 {
    let dist = Uniform::from(1..100);
    let num: i32 = dist.sample(&mut thread_rng()) as i32;
    num
}

fn write_to_file(size: usize, method: &str, elapsed: u128, total: i32) {
    let output = format!("[RustVector]#{:?}#{:?}#{:?}#{:?}\n", 
                         size, method, elapsed, total);
    println!("{}",output);
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open("loging.log")
        .unwrap();

    file.write_all(output.as_bytes()).expect("Fail to write file.");
}