#!/bin/bash

cargo clean
cargo build --release

for size in 1 2 3 4
do


  rm -rf loging.log
  echo "datastructure#initialization#size#inittime#addtime#totaltime" > loging.log

	for counter in 1 2 3 4 5
	do



    # write the first line


    for initialization in true false
    do

        echo "Adding $size elements with initialization $initialization in Run number  $counter "

        # Server experiment
        time taskset -c 0 cargo run $size $initialization
        # Loacal experiment
        # time cargo run $size $initialization
    done


	done
  cat loging.log  >  result/resultRustAllocString_"$size".txt
done
