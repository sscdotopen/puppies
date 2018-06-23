#!/usr/bin/env bash

NUM_RUNS=6

for i in $(seq 0 $NUM_RUNS);
do
cargo run --release /home/ssc/Entwicklung/projects/puppies-experiments/datasets/twitter.csv 8094909 3070055 8 500000 10 \
 | grep ^LOG | cut -d, -f3 > /home/ssc/Entwicklung/projects/puppies-experiments/logs/rust/twitter-$i.csv
done