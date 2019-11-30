#!/bin/bash


mvn clean compile assembly:single

for size in 1 2 3 4
do

  echo "datastructure#initialization#size#inittime#addtime#totaltime" > loging.log
  rm -rf loging.log

	for counter in 1 2 3 4 5
	do



    # write the first line


    for initialization in true flase
    do

	    echo "Adding $size elements with initialization $initialization in Run number  $counter "

	    time taskset -c 0 java -Xms1g -Xmx5g   -cp  ./target/java-memory-alloc-manual-1.0-SNAPSHOT-jar-with-dependencies.jar  allocation/ElementsAddition   $size $initialization


    done

    cat loging.log  >>  result/resultJavaAlloc_"$size".txt
	done

done
