#!/bin/bash
# cargo clean
cargo build --release


# n_thread=10
n_thread=8
# n_thread=4

train_file=../../data/WikipediaPagesOneDocPerLine100k.txt
test_file=../../data/TestingData.txt

# train_file=../../data/SmallTrainingData.txt
# test_file=../../data/TestingData.txt

train_p_directory=../../data/trainPartition/
test_p_directory=../../data/testPartition/

rm -f ../../data/trainPartition/* 
rm -f ../../data/testPartition/*
rm -f result/*
rm -f prediction/*
rm -f serialized/*

n_lines_train=$(< $train_file wc -l)
n_lines_test=$(< $test_file wc -l) 

header="datastructure#method#strategy#k#n_neighbors#n_batch"
for ((i=0; i<$n_thread; i++))
do
    header="$header#n_line_train$i"
done

for ((i=0; i<$n_thread; i++))
do
    header="$header#n_line_test$i"
done

for ((i=0; i<$n_thread; i++))
do 
    header="$header#load_time$i"
done
for ((i=0; i<$n_thread; i++))
do
    header="$header#preprocess_time$i"
done
for ((i=0; i<$n_thread; i++))
do
    header="$header#query_time$i"
done
for ((i=0; i<$n_thread; i++))
do
    header="$header#combine_time$i"
done

header="$header#total"
echo $header > loging.log


n_base_line_train=$((n_lines_train / n_thread))
remainder_train=$((n_lines_train % n_thread))

n_base_line_test=$((n_lines_test / n_thread))
remainder_test=$((n_lines_test % n_thread))
if [ $remainder_train != 0 ]
then
    n_base_line_train=$((n_base_line_train+1))
fi
if [ $remainder_test != 0 ]
then
    n_base_line_test=$((n_base_line_test+1))
fi

split -l $n_base_line_train $train_file $train_p_directory
split -l $n_base_line_test $test_file $test_p_directory

train_partitions=../../data/trainPartition/*
test_partitions=../../data/testPartition/*

declare -a train_p_sizes
for train_p in $train_partitions
do
    train_p_sizes+=($(< $train_p wc -l))
done

declare -a test_p_sizes
for test_p in $test_partitions
do
    test_p_sizes+=($(< $test_p wc -l)) 
done

for method in 1 2
do  
    for strategy in 1 2
    do
    	for n_batch in 2 3
    	do
            for k in 15000 20000 25000
            do
                for counter in 1 2 3 4 5
                do
                    # rm -rf profile/nbatch"$n_batch"_k"$k"_nneighbors"$n_neighbors".txt 
                    time cargo run --release $method $strategy $k 30 $n_batch $train_partitions $test_partitions "${train_p_sizes[@]}" "${test_p_sizes[@]}"
                    # ls -l serialized/ | awk '{if(NR!=1){print $5"#"$9}}' >>  profile/method"$method"_nbatch"$n_batch"_k"$k".txt
                    # rm serialized/*
                done
            done
        done
   done
done





