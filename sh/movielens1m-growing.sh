#!/usr/bin/env bash

NUM_RUNS=6
NUM_SNAPSHOTS=100

for r in $(seq 0 $NUM_RUNS);
do
  rm /home/ssc/Entwicklung/projects/puppies-experiments/logs/rust/movielens1m-growing-$r.csv
  for i in $(seq 0 $NUM_SNAPSHOTS);
  do
    NUM_INTERACTIONS=$(wc -l /home/ssc/Entwicklung/projects/puppies-experiments/datasets/growing/movielens/$i.csv|cut -d" " -f1)
    cargo run --release /home/ssc/Entwicklung/projects/puppies-experiments/datasets/growing/movielens/$i.csv 9746 6040 8 $NUM_INTERACTIONS 10 \
      | grep ^LOG | cut -d, -f3 >> /home/ssc/Entwicklung/projects/puppies-experiments/logs/rust/movielens1m-growing-$r.csv
  done
done