#!/bin/bash

NUM_REPETITIONS=10
EXECUTION_TIME=$(date +%d_%m_%Y_%H_%M);

for i in $(seq 1 $NUM_REPETITIONS);
do
  cargo run --release /home/ssc/Entwicklung/projects/incremental-cooccurrences/src/main/resources/ml20m.csv 138493 27278 8|tee logs/movielens20m-$EXECUTION_TIME-$i.txt
  grep ms logs/movielens20m-$EXECUTION_TIME-$i.txt|grep -v Overall|cut -d' ' -f2|sed s/ms//g|head -n 100 > logs/movielens20m-$EXECUTION_TIME-$i.csv
done