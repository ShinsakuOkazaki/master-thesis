
fn main() {
    let args: Vec<String> = env::args().collect();
    let size: usize = args[1].parse().unwrap();
    let method: i32 = args[2].parse().unwrap();
    run_experiment(size, method);
}

