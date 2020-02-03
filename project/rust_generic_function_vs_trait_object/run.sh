cargo clean
cargo build --release
for size in 10000000 15000000 100000000 150000000
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
<<<<<<< HEAD
            #time taskset -c 0 cargo run --release $size $method $eltype $mutability
            # Loacal experiment
            time cargo run --release $size $method $eltype $mutability
=======
            time taskset -c 0 cargo run $size $method $eltype $mutability
            # Loacal experiment
            # time cargo run $size $method $eltype $mutability
>>>>>>> eb5a8d2941dda9c7fa7e4c9f1c877028b0557b8d
        done
    done
    cat loging.log  >  result/resultRustGenericVsTrait_"$size".txt
done
