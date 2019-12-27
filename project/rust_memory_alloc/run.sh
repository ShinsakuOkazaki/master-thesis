#!/bin/bash

cargo clean
cargo build

for size in 1 2 3 4
do


  rm -rf loging.log
  echo "datastructure#elementtype#initialization#size#inittime#addtime#totaltime" > loging.log

	for counter in 1 2 3 4 5
	do



    # write the first line


    for initialization in true false
    do
        for elementtype in 1 2 3 4
        do
            echo "Adding $size elements with initialization $initialization in Run number  $counter "

            # Server experiment
            # time taskset -c 0 cargo run $size $initialization $elementtype
            # Loacal experiment
            time cargo run $size $initialization $elementtype
        done
    done


	done
  cat loging.log  >  result/resultRustAlloc_"$size".txt
done
