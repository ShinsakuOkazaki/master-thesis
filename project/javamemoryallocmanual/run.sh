#!/bin/bash


mvn clean compile assembly:single

for size in 1 2 3 4
do


  rm -rf loging.log
  echo "datastructure#initialization#size#inittime#addtime#totaltime" > loging.log

	for counter in 1 2 3 4 5
	do



    # write the first line


    for initialization in true flase
    do

	    echo "Adding $size elements with initialization $initialization in Run number  $counter "

      # Server experiment
	    time taskset -c 0 java -Xms1g -Xmx5g   -cp  ./target/java-memory-alloc-manual-1.0-SNAPSHOT-jar-with-dependencies.jar  allocation/ElementsAddition   $size $initialization

	    # Loacal experiment
	    # time java -Xms1g -Xmx5g   -cp  ./target/java-memory-alloc-manual-1.0-SNAPSHOT-jar-with-dependencies.jar  allocation/ElementsAddition   $size $initialization


    done


	done
  cat loging.log  >  result/resultJavaAlloc_"$size".txt
done
