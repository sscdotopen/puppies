#!/usr/bin/env bash

NUM_RUNS=6

for i in $(seq 0 $NUM_RUNS);
do
cargo run --release /home/ssc/Entwicklung/projects/puppies-experiments/datasets/ml1m-shuffled.csv 9746 6040 8 10000 10 \
 | grep ^LOG | cut -d, -f3 > /home/ssc/Entwicklung/projects/puppies-experiments/logs/rust/movielens1m-$i.csv
done