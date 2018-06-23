#!/usr/bin/env bash
NUM_RUNS=6

for i in $(seq 0 $NUM_RUNS);
do
  cargo run --release /home/ssc/Entwicklung/projects/puppies-experiments/datasets/dblp-shuffled.csv 1314051 1314051 8 250000 10 \
   | grep ^LOG | cut -d, -f3 > /home/ssc/Entwicklung/projects/puppies-experiments/logs/rust/dblp-$i.csv
done