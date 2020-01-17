#!/bin/bash

cargo clean
cargo build --release

for size in 1 2 3 4
do


  rm -rf loging.log
  echo "datastructure#size#inittime#addtime#totaltime" > loging.log

	for counter in 1 2 3 4 5
	do



    # write the first line


        echo "Adding $size elements in Run number  $counter "

        # Server experiment
        time taskset -c 0 cargo run $size $initialization
        # Loacal experiment
        # time cargo run $size $initialization
    


	done
  cat loging.log  >  result/resultRustImmutableInteger_"$size".txt
done
