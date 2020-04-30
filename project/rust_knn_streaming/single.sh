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



# time cargo run --release 1 1 25000 30 3 $train_partitions $test_partitions "${train_p_sizes[@]}" "${test_p_sizes[@]}"
# sleep 10m
# time cargo run --release 2 1 25000 30 3 $train_partitions $test_partitions "${train_p_sizes[@]}" "${test_p_sizes[@]}"
#sleep 10m
# time cargo run --release 1 2 25000 30 3 $train_partitions $test_partitions "${train_p_sizes[@]}" "${test_p_sizes[@]}"
#sleep 10m
time cargo run --release 2 2 25000 30 3 $train_partitions $test_partitions "${train_p_sizes[@]}" "${test_p_sizes[@]}"
