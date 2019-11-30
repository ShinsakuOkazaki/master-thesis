#!/bin/bash


mvn clean compile assembly:single

for size in 1 2 3 4
do


	for counter in 1 2 3 4 5
	do
		rm -rf loging.log


    # write the first line
    echo "datastructure#initialization#size#inittime#addtime#totaltime" > loging.log

    for initialization in true flase
    do

	    echo "Adding $size elements with initialization $initialization in Run number  $counter "

	    time taskset -c 0 java -Xms15g -Xmx30g   -cp  ./target/java-memory-alloc-manual-1.0-SNAPSHOT-jar-with-dependencies.jar  allocation/ElementsAddition   $size $initialization

    done
	  cat loging.log  >  result/resultJavaAlloc_"$size"_"$counter".txt
	done

done