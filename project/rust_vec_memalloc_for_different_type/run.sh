#!/bin/bash

cargo clean
cargo build --release
# for size in 3750000 3760000 3770000 3780000

for init in true false
do
	case "$init" in
		"true")
			sizes=(10000000 12500000 15000000 17500000 20000000 )
			;;
		"false")
			sizes=(10000000 12500000 15000000 17500000 20000000 )

			;;
	esac
	for size in "${sizes[@]}"
	do
		rm -rf loging.log
		echo "datastructure#init#size#field#createtime#accesstime#totaltime" > loging.log

		for field in 1 2 3
		do

			for counter in 1 2 3 4 5
			do
				rm -rf string.log
				# write the first line


				echo "Size: $size, Init: $init, Field: $field, Counter: $counter"

				# Server experiment
				time taskset -c 0 cargo run --release $size $init $field
				# Loacal experiment
				# time cargo run --release $size $method $init $field
			done
		done
		cat loging.log  >  result/resultRustOwnerType_init_"$init"_"$size".txt
	done
done
cat string.log  >  result/vectorSize.txt
