#!/bin/bash

cargo clean
cargo build --release
for size in 1000000 1500000 10000000 15000000
do
    rm -rf loging.log
    echo "datastructure#size#field#createtime#accesstime#totaltime#count" > loging.log

    for field in 1 2 3
    do
        
        for counter in 1 2 3 4 5
        do

        # write the first line


            echo "Adding $size elements in Run number  $counter "

            # Server experiment
            # stime taskset -c 0 cargo run $size $field
            # Loacal experiment
            time cargo run $size $method $field
        done
    done
    cat loging.log  >  result/resultRustOwnerType_"$size".txt
done
