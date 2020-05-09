#!/bin/bash

cargo clean
cargo build --release
# for size in 3900000 3910000 3920000 3930000 3940000


for size in 10000000  20000000  30000000 40000000 
do
    rm -rf loging.log
    echo "datastructure#size#field#createtime#accesstime#droptime#totaltime" > loging.log

    for field in 2 3
    do
        
        for counter in 1 2 3 4 5
        do
            rm -rf string.log
        # write the first line


            echo "Size: $size, Field: $field, Counter: $counter"

            # Server experiment
            time taskset -c 0 cargo run --release $size $field
            # Loacal experiment
            # time cargo run --release $size $field
        done
    done
cat loging.log  >  result/resultRustRc_"$size".txt
done


