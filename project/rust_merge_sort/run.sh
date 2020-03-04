#!/bin/bash

cargo clean
cargo build --release


for size in 100000 150000 200000
do
    rm -rf loging.log
    echo "datastructure#size#method#sorttime" > loging.log

    for method in 1 2 3
    do
        
        for counter in 1 2 3 4 5
        do
           # rm -rf string.log
        # write the first line


            echo "Size: $size, Method: $method, Counter: $counter"

            # Server experiment
            # time cargo run --release $size $method
            # Loacal experiment
            time cargo run --release $size $method
        done
    done
cat loging.log  >  result/resultMergesort_"$size".txt
done
