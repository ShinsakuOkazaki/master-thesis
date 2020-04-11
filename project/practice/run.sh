cargo clean
cargo build --release

for size in 100 1000 10000 100000
do
    rm -rf loging.log
    echo "datastructure#k#n_neighbors#threadtime#globaltime" > loging.log

    for method in 1 2
    do
        for k in 10000 20000
        do
            for n_neighbors in 10 20 30
            do
                time cargo run --release  $method $k $n_neighbors ~/master-thesis/data/TestingData.txt  ~/master-thesis/data/trainPartition/*
            done
        done
    done
cat loging.log > result/resultKnn_"$size".txt
done
