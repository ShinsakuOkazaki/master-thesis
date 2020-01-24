#!/bin/bash

cargo clean
cargo build --release
for size in 1000000 1500000 10000000 15000000
do
    rm -rf loging.log
    echo "datastructure#method#type#mutability#size#inittime#addtime#accesstime#totaltime" > loging.log

    for method in 1 2 3 4
    do
        for eltype in 1 2
        do
            for mutability in true false
            do
                for counter in 1 2 3 4 5
                do



                # write the first line


                    echo "Adding $size elements in Run number  $counter "

                    # Server experiment
                    time taskset -c 0 cargo run $size $method $eltype $mutability
                    # Loacal experiment
                    #time cargo run $size $method $eltype $mutability
                done
            done
        done
    done
    cat loging.log  >  result/resultRustVarious_"$size".txt
done
