#!/bin/bash

NUM_REPETITIONS=10
EXECUTION_TIME=$(date +%d_%m_%Y_%H_%M);

for i in $(seq 1 $NUM_REPETITIONS);
do
  cargo run --release datasets/bookcrossing-shuffled.csv 445802 105279 8|tee logs/bookcrossing-$EXECUTION_TIME-$i.txt
  grep ms logs/bookcrossing-$EXECUTION_TIME-$i.txt|grep -v Overall|cut -d' ' -f2|sed s/ms//g > logs/bookcrossing-$EXECUTION_TIME-$i.csv
done