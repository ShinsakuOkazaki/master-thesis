use std::thread;
use std::path::Path;
pub use std::sync::Arc;
use crate::objects::customer::*;
use crate::objects::access::*;
use std::collections::HashMap;

const MAX_THREADS: usize = 4;

fn tree_aggregate(paths: Arc<Vec<String>>, left: usize, right: usize, depth: usize) -> HashMap<String, Vec<Arc<CustomerOwned>>>
{   
    let mut agg_current;
    let arr;
    if right - left > 1 {
        let mid = (left + right) / 2;
        let new_depth = depth + 1;
        let paths_cloned1 = Arc::clone(&paths);
        let paths_cloned2 = Arc::clone(&paths);
        let mut agg_left;
        let mut agg_right;
        let path = &paths[mid];
        if new_depth < MAX_THREADS {
            let (sender1, receiver1) = crossbeam::channel::unbounded();
            let (sender2, receiver2) = crossbeam::channel::unbounded(); 
        
            let _ = thread::spawn(move || {
               let agg = tree_aggregate(paths_cloned1, left, mid, new_depth);
               sender1.send(agg).unwrap();
            });
            
            let _ = thread::spawn(move || {
                let agg = tree_aggregate(paths_cloned2, mid, right, new_depth);
                sender2.send(agg).unwrap();
             });

             arr = deserialize_vector_arc(path).unwrap();
             agg_current = aggregate_local(&arr[..]);
             agg_left = receiver1.recv().unwrap();
             agg_right = receiver2.recv().unwrap();
        } else {
            arr = deserialize_vector_arc(path).unwrap();
            agg_current = aggregate_local(&arr[..]);
            agg_left = tree_aggregate(paths_cloned1, left, mid, new_depth);
            agg_right = tree_aggregate(paths_cloned2, mid, right, new_depth);
        }
        return tree_join(agg_current, agg_left, agg_right);
    } else {
        let mid = (right - left) / 2;
        let path = &paths[mid]; 
        arr = deserialize_vector_arc(path).unwrap();
        return aggregate_local(&arr[..]);
    }    
}

fn tree_join(mut agg_current: HashMap<String, Vec<Arc<CustomerOwned>>>, 
            mut agg_left: HashMap<String, Vec<Arc<CustomerOwned>>>, 
            mut agg_right: HashMap<String, Vec<Arc<CustomerOwned>>>) -> HashMap<String, Vec<Arc<CustomerOwned>>>
{

    
    for (last_name, mut customers) in agg_left {
        let vector = agg_current.entry(last_name).or_insert_with(Vec::new);
        vector.append(&mut customers); 
    }

    for (last_name, mut customers) in agg_right {
        let vector = agg_current.entry(last_name).or_insert_with(Vec::new);
        vector.append(&mut customers); 
    }
    return agg_current;
}


fn aggregate_local(arr :&[Arc<CustomerOwned>]) -> HashMap<String, Vec<Arc<CustomerOwned>>>
{   
    let mut agg = HashMap::new();
    let n = arr.len();
    for i in 0..n {
        let customer = Arc::clone(&arr[i]);
        let last_name = customer.last_name.clone();
        let vector = agg.entry(last_name).or_insert_with(Vec::new);
        vector.push(customer);
    }
    return agg;
}

