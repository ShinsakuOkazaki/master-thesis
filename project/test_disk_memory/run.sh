(time cargo run ../data/input1.csv output/output1.csv) 2>&1 | tee -a log.txt
(time cargo run ../data/input2.csv output/output2.csv) 2>&1 | tee -a log.txt
(time cargo run ../data/input3.csv output/output3.csv) 2>&1 | tee -a log.txt
