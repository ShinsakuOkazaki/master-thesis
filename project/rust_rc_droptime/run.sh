#!/bin/bash

# cargo clean
cargo build --release
# for size in 3900000 3910000 3920000 3930000 3940000


# for size in 10000000  20000000  30000000 40000000 
line_sizes=(10000000 20000000 30000000)
order_sizes=(2500000 5000000 7500000)
customer_sizes=(250000 500000 750000)

line_file=../data/tables_scale_0.1/lineitem-large.tbl
order_file=../data/tables_scale_0.1/orders-large.tbl
customer_file=../data/tables_scale_0.1/customer-large.tbl

for size_idx in 0 1 2
do
    rm -rf loging.log
    echo "datastructure#size#field#createtime#accesstime#droptime#totaltime" > loging.log
    line_size=${line_sizes[$size_idx]}
    order_size=${order_sizes[$size_idx]}
    customer_size=${customer_sizes[$size_idx]}

    for field in 1 2 3
    do
        
        for counter in 1 2 3 4 5
        do
            rm -rf string.log
        # write the first line


            echo "LinesSize: $line_size, OrderSize: $order_size, CustomerSize: $customer_size, Field: $field, Counter: $counter"

            # Server experiment
            time taskset -c 0 cargo run --release $line_file $order_file $customer_file $line_size $order_size $customer_size $field
            # Loacal experiment
            # time cargo run --release $line_file $order_file $customer_file $line_size $order_size $customer_size $field
            
        done
    done
cat loging.log  >  result/resultRustRc_"$size".txt
done


