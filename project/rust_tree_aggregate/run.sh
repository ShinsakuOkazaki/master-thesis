#!/bin/bash

cargo clean
cargo build --release


for size in 1000 2000 3000 4000
do
    rm -rf loging.log
    echo "datastructure#size#partnumber#method#aggtime#accesstime" > loging.log

    for partnumber in 7 15 31
    do
        for method in 1 2
        do
            
            for counter in 1 2 3 4 5
            do
            rm -rf string.log
            # write the first line


                echo "Size: $size, Partnumber: $partnumber, Method: $method, Counter: $counter"

                # Server experiment
                time cargo run --release $size $partnumber $method
                # Loacal experiment
                # time cargo run --release $size $partnumber $method
                rm partition/*
            done
        done
    done
cat loging.log  >  result/resultTreeAggregate_"$size".txt
done
