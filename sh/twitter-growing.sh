#!/usr/bin/env bash

NUM_RUNS=7
NUM_SNAPSHOTS=88

for r in $(seq 0 $NUM_RUNS);
do
  rm /home/ssc/Entwicklung/projects/puppies-experiments/logs/rust/twitter-growing-$r.csv
  for i in $(seq 0 $NUM_SNAPSHOTS);
  do
    NUM_INTERACTIONS=$(wc -l /home/ssc/Entwicklung/projects/puppies-experiments/datasets/growing/twitter/$i.csv | cut -d" " -f1)
    cargo run --release /home/ssc/Entwicklung/projects/puppies-experiments/datasets/growing/twitter/$i.csv 8094909 3070055 8 $NUM_INTERACTIONS 10 \
      | grep ^LOG | cut -d, -f3 >> /home/ssc/Entwicklung/projects/puppies-experiments/logs/rust/twitter-growing-$r.csv
  done
done