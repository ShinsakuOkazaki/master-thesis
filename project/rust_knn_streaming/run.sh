#!/bin/sh
cargo clean
cargo build --release

time cargo run --release 1 20000 20 3 /home/ubuntu/research/master-thesis/data/trainPartition/*  /home/ubuntu/research/master-thesis/data/testPartition/* 1873 1873 1873 1873 1873 1873 1873 1873 1874 1867 10000 10000 10000 10000 10000 10000 10000 10000 10000 10000 
