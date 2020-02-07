#!/bin/bash

cargo clean
cargo build --release
# for size in 3750000 3760000 3770000 3780000
for size in 37500 37600 37700 37800
do
    rm -rf loging.log
    echo "datastructure#init#size#field#createtime#accesstime#totaltime" > loging.log

    for init in true false
    do
        for field in 1 2 3
        do
            
            for counter in 1 2 3 4 5
            do
                rm -rf string.log
            # write the first line


                echo "Adding $size elements in Run number  $counter "

                # Server experiment
                time taskset -c 0 cargo run --release $size $init $field
                # Loacal experiment
                # time cargo run --release $size $method $init $field
            done
        done
    done
    cat loging.log  >  result/resultRustOwnerType_"$size".txt
done
cat string.log  >  result/vectorSize.txt
