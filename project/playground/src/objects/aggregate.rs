use std::thread;
use std::path::Path;
pub use std::sync::Arc;
use crate::objects::customer::*;
use crate::objects::access::*;
const MAX_THREADS: usize = 4;

fn tree_aggregate_sum<P: AsRef<Path>>(arr: Arc<Vec<P>>, left: usize, right: usize, depth: usize)
{   
    let sum_current;
    let path;
    if right - left > 1 {
        let mid = (left + right) / 2;
        let new_depth = depth + 1;
        let arr_cloned1 = Arc::clone(&arr);
        let arr_cloned2 = Arc::clone(&arr);
        let sum_left;
        let sum_right;
        path = arr[mid];
        if new_depth < MAX_THREADS {
            let (sender1, receiver1) = crossbeam::channel::unbounded();
            let (sender2, receiver2) = crossbeam::channel::unbounded(); 
        
            let _ = thread::spawn(move |_| {
               let sum = tree_aggregate_sum(arr_cloned1, left, mid, new_depth);
               sender1.send(sum).unwrap();
            });
            
            let _ = thread::spawn(move |_| {
                let sum = tree_aggregate_sum(arr_cloned2, mid, right, new_depth);
                sender2.send(sum).unwrap();
             });
             sum_current = add_local(path);
             sum_left = receiver1.recv().unwrap();
             sum_right = receiver2.recv().unwrap();
        } else {
            sum_current = add_local(path);
            sum_left = tree_aggregate_sum(arr_cloned1, left, mid, new_depth);
            sum_right = tree_aggregate_sum(arr_cloned2, mid, right, new_depth);
        }
        return tree_reduce_sum(sum_current, sum_left, sum_right);
    } else if (right - left == 1) {
        let mid = (right - left) / 2;
        path = arr[mid];
        sum_current = add_local(path);
        return tree_aggregate_sum(sum_current, 0, 0);
    }    
}

fn tree_reduce_sum<T>(sum_current: i32,  sum_left: i32, sum_right: i32) 
{
    let sum =  sum_left + sum_current + sum_right;
    sum
}

fn add_local<P: AsRef<Path>>(path: P) {
    let customers = deserialize_vector(file).unwrap();
    let sum_current = add_customer_key(&customers[..]);
    sum_current
}

fn add_customer_key<T>(customers: &[T]) 
    where T: Clone + Customer + PartialOrd + Send + Sync + Default
{
    let mut sum = 0;
    for i in 0..customers.len() {
        sum += customers[i].key;
    }
    sum
}