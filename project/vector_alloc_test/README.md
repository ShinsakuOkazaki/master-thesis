## Memory Allocation Experiment for Rust Vector.

# Objection
This project is to experiment how static and dynamic memory allocation of Rust behaves. 
For the assessment, Element addition to Vector is emploied here. 
There are two parameters; initial size of Vector and final size of Vector after additions of element.
We are interested in the impact of memory allocation by Vector initial allocation and expantion to runtime.

# Experiment
First, an Vector is created with specified initial size. Then, in a loop, a element is added for each iteration until the size of the Vector gets the final size. When Vector hits current limit of its size and expands the limit, it doubles the current size. This means that the expantion of Vector size might affect the digradation of runtime performance.

# Criterion Benchmark Library for Rust
We use Criterion to perform Rust benchmark. We set parameter.

# Run Experiment 
```
cargo bench --bench my_benchmark -- --verbose
```
