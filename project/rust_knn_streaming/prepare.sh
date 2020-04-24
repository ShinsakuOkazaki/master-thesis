#!/bin/bash

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

header="datastructure#method#k#n_neighbors#n_batch"
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
    header="$header#preprocess_time$i"
done
for ((i=0; i<$n_thread; i++))
do
    header="$header#query_time$i"
done
for ((i=0; i<$n_thread; i++))
do
    header="$header#predicition_time$i"
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


