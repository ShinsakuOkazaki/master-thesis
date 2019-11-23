## Memory Allocation Experiment for Java ArrayList.

# Objection
This project is to experiment how static and dynamic memory allocation of Java behaves. 
For the assessment, Element addition to ArrayList is emploied here. 
There are two parameters; initial size of ArrayList and final size of ArrayList after additions of element.
We are interested in the impact of memory allocation by ArrayList initial allocation and expantion to runtime.

# Experiment
First, an ArrayList is created with specified initial size. Then, in a loop, a element is added for each iteration until the size of the ArrayList gets the final size. When ArrayList hits current limit of its size and expands the limit, it doubles the current size. This means that the expantion of ArrayList size might affect the digradation of runtime performance.

# Java Microbenchmark Hardness (JMH)
We use JMH to perform Java benchmark. We set parameter listed below.
- Benchmark Mode: Average Time (Nanoseconds)
- Warm up Iteration: 2
- Number of Fork: 2
- Size of Memory: 2GB

# Run Experiment 
See [jmh-tutorial on github](https://github.com/guozheng/jmh-tutorial/blob/master/README.md).
```
mvn clean install
# Store result in json file.
java -jar target/benchmarks.jar -rf json
```


