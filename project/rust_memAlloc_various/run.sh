#!/bin/bash

cargo clean
cargo build --release

for size in 1000 10000 100000 1000000
do
    rm -rf loging.log
    echo "datastructure#method#type#size#inittime#addtime#totaltime" > loging.log

    for method in 1 2 3 4
    do
        for eltype in 1 2
        do
            
            for counter in 1 2 3 4 5
            do



            # write the first line


                echo "Adding $size elements in Run number  $counter "

                # Server experiment
                time taskset -c 0 cargo run $size $method $eltype
                # Loacal experiment
                # time cargo run $size $method $eltype
            done
        done
    done
    cat loging.log  >  result/resultRustVarious_"$size".txt
done
