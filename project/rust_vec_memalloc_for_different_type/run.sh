#!/bin/bash

cargo clean
cargo build --release
# for size in 3750000 3760000 3770000 3780000

for init in true false
do 
    case "$init" in 
        "true") 
            sizes=( 3900000 3910000 3920000 3930000 3940000 )
        ;;
        "false")
            sizes=( 3750000 3760000 3780000 3790000 3800000 )
        ;;
    esac
    for size in "${sizes[@]}"
    do
        rm -rf loging.log
        echo "datastructure#init#size#field#createtime#accesstime#totaltime" > loging.log

        for field in 1 2 3
        do
            
            for counter in 1 2 3 4 5
            do
                rm -rf string.log
            # write the first line


                echo "Size: $size, Init: $init, Field: $field, Counter: $counter"

                # Server experiment
                time taskset -c 0 cargo run --release $size $init $field
                # Loacal experiment
                # time cargo run --release $size $method $init $field
            done
        done
    cat loging.log  >  result/resultRustOwnerType_init_"$init"_"$size".txt
    done
done
cat string.log  >  result/vectorSize.txt
