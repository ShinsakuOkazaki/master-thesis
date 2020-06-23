#!/bin/bash


cargo clean 
cargo build --release

part_file=../data/tables_scale_0.1/part-large.tbl
sizes=(900000)

for size_idx in 0
do
    rm -f loging.log
    echo "datastructure#method#size#ser_time#deser_time" > loging.log
    size=${sizes[$size_idx]}
    for method in 1 2 3 4
    do
        for counter in 1 2 3 4 5
        do
            rm -f serialized_with*

            echo "Method: $method, Size: $size, Counter: $counter"

            # Server experiment
            time taskset -c 0 cargo run --release $part_file $size $method
            # Loacal experiment
            # time cargo run --release $part_file $size $method
        done
    done
    cat loging.log  >  result/resultSerialization_"$size".txt
done
