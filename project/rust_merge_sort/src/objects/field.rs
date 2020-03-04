use rand::{Rng,SeedableRng};
use rand::rngs::StdRng;
use rand::distributions::Alphanumeric;
use rand::distributions::{Uniform, Distribution};
//use std::rc::Rc;
use std::iter;
//use std::sync::Arc;
// Function to get string vector.
// All vector will have the same elements with length of 5.
pub fn get_string_vector(size: usize) -> Vec<String> {
    let mut strings = Vec::with_capacity(size);
    // Set random seed.
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    for _ in 0..size {
        // Get random Stirng whose length is 5.
        let string: String = iter::repeat(())
                            .map(|()| rng.sample(Alphanumeric))
                            .take(5)
                            .collect();
        strings.push(string);
    }
    strings
}

// pub fn get_string_rc_vector(size: usize) -> Vec<Rc<String>> {
//     let mut strings = Vec::with_capacity(size);
//     let mut rng: StdRng = SeedableRng::seed_from_u64(0);
//     for _ in 0..size {
//         // Get random Stirng whose length is 5.
//         let string: String = iter::repeat(())
//                             .map(|()| rng.sample(Alphanumeric))
//                             .take(5)
//                             .collect();
//         strings.push(Rc::new(string));
//     }
//     strings
// }


// pub fn get_string_arc_vector(size: usize) -> Vec<Arc<String>> {
//     let mut strings = Vec::with_capacity(size);
//     let mut rng: StdRng = SeedableRng::seed_from_u64(0);
//     for _ in 0..size {
//         // Get random Stirng whose length is 5.
//         let string: String = iter::repeat(())
//                             .map(|()| rng.sample(Alphanumeric))
//                             .take(5)
//                             .collect();
//         strings.push(Arc::new(string));
//     }
//     strings
// }

pub fn get_integer_vector(size: usize) -> Vec<i32> {
    let mut integers = Vec::with_capacity(size);
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    let dist = Uniform::from(1..1000);
    let mut num: i32;
    for _ in 0..size {
        num = dist.sample(&mut rng) as i32;
        integers.push(num);
    }
    integers    
}

// pub fn get_integer_rc_vector(size: usize) -> Vec<Rc<i32>>  {
//     let mut integers = Vec::with_capacity(size);
//     let mut rng: StdRng = SeedableRng::seed_from_u64(0);
//     let dist = Uniform::from(1..1000);
//     for _ in 0..size {
//         let num = dist.sample(&mut rng) as i32;
//         integers.push(Rc::new(num));
//     }
//     integers
// }

// pub fn get_integer_arc_vector(size: usize) -> Vec<Arc<i32>>  {
//     let mut integers = Vec::with_capacity(size);
//     let mut rng: StdRng = SeedableRng::seed_from_u64(0);
//     let dist = Uniform::from(1..1000);
//     for _ in 0..size {
//         let num = dist.sample(&mut rng) as i32;
//         integers.push(Arc::new(num));
//     }
//     integers
// }


pub fn get_float_vector(size: usize) -> Vec<f64> {
    let mut floats = Vec::with_capacity(size);
    let mut rng: StdRng = SeedableRng::seed_from_u64(0);
    let dist = Uniform::from(0..1);
    let mut num: f64;
    for _ in 0..size {
        num = dist.sample(&mut rng) as f64;
        floats.push(num);
    }
    floats
}

// pub fn get_float_rc_vector(size: usize) -> Vec<Rc<f64>> {
//     let mut floats = Vec::with_capacity(size);
//     let mut rng: StdRng = SeedableRng::seed_from_u64(0);
//     let dist = Uniform::from(0..1);
//     for _ in 0..size {
//         let num = dist.sample(&mut rng) as f64;
//         floats.push(Rc::new(num));
//     }
//     floats
// }

// pub fn get_float_arc_vector(size: usize) -> Vec<Arc<f64>> {
//     let mut floats = Vec::with_capacity(size);
//     let mut rng: StdRng = SeedableRng::seed_from_u64(0);
//     let dist = Uniform::from(0..1);
//     for _ in 0..size {
//         let num = dist.sample(&mut rng) as f64;
//         floats.push(Arc::new(num));
//     }
//     floats
// }

