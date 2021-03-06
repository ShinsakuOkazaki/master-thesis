#!/bin/bash

cargo clean
cargo build --release


for size in 4000000 8000000 12000000  16000000 
do
    rm -rf loging.log
    echo "datastructure#size#method#sorttime" > loging.log

    for method in 1 2
    do
        
        for counter in 1 2 3 4 5
        do
           rm -rf string.log
        # write the first line


            echo "Size: $size, Method: $method, Counter: $counter"

            # Server experiment
            time cargo run --release $size $method
            # Loacal experiment
            # time cargo run --release $size $method
        done
    done
cat loging.log  >  result/resultMergesort_"$size".txt
done
cat string.log  >  result/vectorSize.txt
