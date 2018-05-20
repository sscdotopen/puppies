#!/bin/bash

NUM_REPETITIONS=10
EXECUTION_TIME=$(date +%d_%m_%Y_%H_%M);

for i in $(seq 1 $NUM_REPETITIONS);
do
  cargo run --release /home/ssc/Entwicklung/projects/incremental-cooccurrences/src/main/resources/ml1m.csv 9746 6040 8|tee logs/movielens-$i.txt
  grep ms logs/movielens-$i.txt|grep -v Overall|cut -d' ' -f2|sed s/ms//g|head -n 100 > logs/movielens-$i.csv
done