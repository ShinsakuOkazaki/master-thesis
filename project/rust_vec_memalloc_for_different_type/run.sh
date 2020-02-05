#!/bin/bash

cargo clean
cargo build --release
for size in 3870000 3880000 3890000 3900000
do
    rm -rf loging.log
    echo "datastructure#size#field#createtime#accesstime#totaltime" > loging.log

    for field in 1 2 3
    do
        
        for counter in 1 2 3 4 5
        do
            rm -rf string.log
        # write the first line


            echo "Adding $size elements in Run number  $counter "

            # Server experiment
            time taskset -c 0 cargo run --release $size $field
            # Loacal experiment
            # time cargo run --release $size $method $field
        done
    done
    cat loging.log  >  result/resultRustOwnerType_"$size".txt
done
