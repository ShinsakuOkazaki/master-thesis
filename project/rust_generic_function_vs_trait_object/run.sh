cargo clean
cargo build --release
for size in 15000000 18000000 21000000 24000000
do
    rm -rf loging.log
    echo "datastructure#size#method#runtime#total" > loging.log

    for method in 1 2
    do
        for counter in 1 2 3 4 5
        do



        # write the first line


            echo "Adding $size elements in Run number  $counter "

            # Server experiment
            time taskset -c 0 cargo run --release $size $method
            # Loacal experiment
            # time cargo run --release $size $method 
        done
    done
    cat loging.log  >  result/resultRustGenericVsTrait_"$size".txt
done
