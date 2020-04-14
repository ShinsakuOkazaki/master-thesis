#!/bin/sh
cargo clean
cargo build --release

#train_file = '/Users/sinsakuokazaki/master-thesis/data/SmallTrainingData.txt'
#train_lines = $((wc -l $train_file))
#echo "train_line: train_file"
cargo run --release 1 20000 20 3 /Users/sinsakuokazaki/master-thesis/data/trainPartition/* /Users/sinsakuokazaki/master-thesis/data/testPartition/* 861 861 861 859 4681 4681 4681 4681
