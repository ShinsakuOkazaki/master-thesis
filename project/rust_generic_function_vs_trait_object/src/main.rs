extern crate rand;
use rand::SeedableRng;
use rand::rngs::StdRng;
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
    let start = Instant::now();
    let mut total = 0;

    {
        let vector = get_triangle_vector(size);
        for i in 0..size {
            total += get_area_trait_object(&*vector[i]);
        }
    }
    
    {
        let vector = get_circle_vector(size);
        for i in 0..size {
            total -= get_area_trait_object(&*vector[i]);
        }
    }
    
    {
        let vector = get_rectangle_vector(size);
        for i in 0..size {
            total += get_area_trait_object(&*vector[i]);
        }
    }

    {
        let vector = get_pentagon_vector(size);
        for i in 0..size {
            total -= get_area_trait_object(&*vector[i]);
        }
    }
    
    {
        let vector = get_parallelogram_vector(size);
        for i in 0..size {
            total += get_area_trait_object(&*vector[i]);
        }
    }

    {
        let vector = get_someshape_a_vector(size);
        for i in 0..size {
            total -= get_area_trait_object(&*vector[i]);
        }
    }

    {
        let vector = get_someshape_b_vector(size);
        for i in 0..size {
            total += get_area_trait_object(&*vector[i]);
        }
    }

    {
        let vector = get_someshape_c_vector(size);
        for i in 0..size {
            total -= get_area_trait_object(&*vector[i]);
        }
    }

    {
        let vector = get_someshape_d_vector(size);
        for i in 0..size {
            total += get_area_trait_object(&*vector[i]);
        }
    }
    
    {
        let vector = get_someshape_e_vector(size);
        for i in 0..size {
            total -= get_area_trait_object(&*vector[i]);
        }
    }
    
    let elapsed = start.elapsed().as_millis();
    write_to_file(size, "trait", elapsed, total);
}

fn run_ex_generic_function(size: usize) {
    let start = Instant::now();
    let mut total = 0;


    {
        let vector = get_triangle_vector(size);
        for i in 0..size {
            total += get_area_generic_function(&*vector[i]);
        }
    }
    
    {
        let vector = get_circle_vector(size);
        for i in 0..size {
            total -= get_area_generic_function(&*vector[i]);
        }
    }

    {
        let vector = get_rectangle_vector(size);
        for i in 0..size {
            total += get_area_generic_function(&*vector[i]);
        }   
    }
    
    {
        let vector = get_pentagon_vector(size);
        for i in 0..size {
            total -= get_area_generic_function(&*vector[i]);
        }
    }
    
    {
        let vector = get_parallelogram_vector(size);
        for i in 0..size {
            total += get_area_generic_function(&*vector[i]);
        }
    }

    {
        let vector = get_someshape_a_vector(size);
        for i in 0..size {
            total -= get_area_generic_function(&*vector[i]);
        }
    }
    
    {   
        let vector = get_someshape_b_vector(size);
        for i in 0..size {
            total += get_area_generic_function(&*vector[i]);
        }
    }

    {
        let vector  = get_someshape_c_vector(size);
        for i in 0..size {
            total -= get_area_generic_function(&*vector[i]);
        }
    }

    {
        let vector = get_someshape_d_vector(size);
        for i in 0..size {
            total += get_area_generic_function(&*vector[i]);
        }
    }
    
    { 
        let vector = get_someshape_e_vector(size);
        for i in 0..size {
            total -= get_area_generic_function(&*vector[i]);
        }
    } 
    
    let elapsed = start.elapsed().as_millis();
    write_to_file(size, "generic", elapsed, total);
}

fn get_area_trait_object(shape: &dyn Shape) -> i32 {
    shape.get_area()
}

fn get_area_generic_function<T: Shape>(shape: &T) -> i32 {
    shape.get_area()
}

fn get_triangle_vector(size: usize) -> Vec<Box<Triangle>> {
    let mut triangle_vector = Vec::with_capacity(size);
    for _ in 0..size {
        let triangle =  Triangle {
            bottom: get_integer(),
            hight: get_integer()
        };
        triangle_vector.push(Box::new(triangle));
    }
   triangle_vector
}

fn get_circle_vector(size: usize) -> Vec<Box<Circle>> {
    let mut circle_vector = Vec::with_capacity(size);
    for _ in 0..size {
        let circle =  Circle {
            r_vertical: get_integer(),
            r_horizontal: get_integer()
        };
        circle_vector.push(Box::new(circle));
    }
    circle_vector
}

fn get_rectangle_vector(size: usize) -> Vec<Box<Rectangle>> {
    let mut rectangle_vector = Vec::with_capacity(size);
    for _ in 0..size {
        let rectangle =  Rectangle {
            wide: get_integer(),
            hight: get_integer()
        };
        rectangle_vector.push(Box::new(rectangle));
    }
    rectangle_vector
}

fn get_pentagon_vector(size: usize) -> Vec<Box<Pentagon>> {
    let mut pentagon_vector = Vec::with_capacity(size);
    for _ in 0..size {
        let pentagon =  Pentagon {
            a: get_integer(),
            h: get_integer()
        };
        pentagon_vector.push(Box::new(pentagon));
    }
    pentagon_vector
}

fn get_parallelogram_vector(size: usize) -> Vec<Box<Parallelogram>> {
    let mut parallelogram_vector = Vec::with_capacity(size);
    for _ in 0..size {
        let parallelogram =  Parallelogram {
            base: get_integer(),
            height: get_integer()
        };
        parallelogram_vector.push(Box::new(parallelogram));
    }
    parallelogram_vector
}

fn get_someshape_a_vector(size: usize) -> Vec<Box<SomeShapeA>> {
    let mut shape_vector = Vec::with_capacity(size);
    for _ in 0..size {
        let shape =  SomeShapeA {
            a: get_integer(),
            b: get_integer(),
            c: get_integer(),
            d: get_integer(),
            e: get_integer()
        };
        shape_vector.push(Box::new(shape));
    }
    shape_vector
}


fn get_someshape_b_vector(size: usize) -> Vec<Box<SomeShapeB>> {
    let mut shape_vector = Vec::with_capacity(size);
    for _ in 0..size {
        let shape =  SomeShapeB {
            a: get_integer(),
            b: get_integer(),
            c: get_integer(),
            d: get_integer(),
            e: get_integer()
        };
        shape_vector.push(Box::new(shape));
    }
    shape_vector
}

fn get_someshape_c_vector(size: usize) -> Vec<Box<SomeShapeC>> {
    let mut shape_vector = Vec::with_capacity(size);
    for _ in 0..size {
        let shape =  SomeShapeC {
            a: get_integer(),
            b: get_integer(),
            c: get_integer(),
            d: get_integer(),
            e: get_integer()
        };
        shape_vector.push(Box::new(shape));
    }
    shape_vector
}

fn get_someshape_d_vector(size: usize) -> Vec<Box<SomeShapeD>> {
    let mut shape_vector = Vec::with_capacity(size);
    for _ in 0..size {
        let shape =  SomeShapeD {
            a: get_integer(),
            b: get_integer(),
            c: get_integer(),
            d: get_integer(),
            e: get_integer()
        };
        shape_vector.push(Box::new(shape));
    }
    shape_vector
}

fn get_someshape_e_vector(size: usize) -> Vec<Box<SomeShapeE>> {
    let mut shape_vector = Vec::with_capacity(size);
    for _ in 0..size {
        let shape =  SomeShapeE {
            a: get_integer(),
            b: get_integer(),
            c: get_integer(),
            d: get_integer(),
            e: get_integer()
        };
        shape_vector.push(Box::new(shape));
    }
    shape_vector
}

pub trait Shape {
    fn get_area(&self) -> i32;
}

pub struct Triangle {
    bottom: i32,
    hight: i32
}

pub struct Circle {
    r_vertical: i32,
    r_horizontal: i32
}

pub struct Rectangle {
    wide: i32,
    hight: i32
}

pub struct Pentagon {
    a: i32, 
    h: i32
}

pub struct Parallelogram {
    base: i32,
    height: i32
}

pub struct SomeShapeA {
    a: i32,
    b: i32,
    c: i32,
    d: i32, 
    e: i32
}

pub struct SomeShapeB {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32
}

pub struct SomeShapeC {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32
}

pub struct SomeShapeD {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32
}

pub struct SomeShapeE {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
    e: i32
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

impl Shape for Pentagon {
    fn get_area(&self) -> i32 {
        5 * self.a * self.h
    }
}

impl Shape for Parallelogram {
    fn get_area(&self) -> i32 {
        self.base * self.height
    }
}

impl Shape for SomeShapeA {
    fn get_area(&self) -> i32 {
        2 * self.a + self.b + self.c + self.d + self.e
    }
}

impl Shape for SomeShapeB {
    fn get_area(&self) -> i32 {
        self.a + 2 * self.b + self.c + self.d + self.e
    }
}

impl Shape for SomeShapeC {
    fn get_area(&self) -> i32 {
        self.a + self.b + 2 * self.c + self.d + self.e
    }
}

impl Shape for SomeShapeD {
    fn get_area(&self) -> i32 {
        self.a + self.b + self.c + 2 * self.d + self.e
    }
}

impl Shape for SomeShapeE {
    fn get_area(&self) -> i32 {
        self.a + self.b + self.c + self.d + 2 * self.e
    }
}


fn get_integer() -> i32 {
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    let dist = Uniform::from(1..5);
    let num: i32 = dist.sample(&mut rng) as i32;
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