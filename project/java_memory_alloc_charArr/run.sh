#!/bin/bash


mvn clean compile assembly:single

for size in 1 2 3 4
do


  rm -rf loging.log
  echo "datastructure#initialization#size#inittime#addtime#totaltime" > loging.log

	for counter in 1 2 3 4 5
	do



    # write the first line


    for initialization in true false
    do

	    echo "Adding $size elements with initialization $initialization in Run number  $counter "

      # Server experiment
	    time taskset -c 0 java -Xms1g -Xmx5g   -cp  ./target/java_memory_alloc_charArr-1.0-SNAPSHOT-jar-with-dependencies.jar  allocation/CharArrAddition   $size $initialization

	    # Loacal experiment
	    # time java -Xms1g -Xmx5g   -cp  ./target/java_memory_alloc_charArr-1.0-SNAPSHOT-jar-with-dependencies.jar  allocation/CharArrAddition    $size $initialization


    done


	done
  cat loging.log  >  result/resultJavaAllocCharArr_"$size".txt
done
